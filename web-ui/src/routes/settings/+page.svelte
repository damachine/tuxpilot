<script lang="ts">
	import { onMount } from 'svelte';
	
	let isLoading = $state(false);
	let statusMessage = $state('Loading configuration...');
	let statusType: 'info' | 'success' | 'warning' | 'error' = $state('info');
	let alerts: Array<{id: string, type: string, message: string}> = $state([]);
	let availableModels: Array<{id: string, name: string}> = $state([]);
	
	let config = $state({
		ai: {
			provider: 'ollama',
			model: 'llama3.2',
			temperature: 0.7,
			max_tokens: 2048,
			api_key_configured: false
		},
		execution: {
			mode: 'supervised',
			require_confirmation: true,
			timeout: 30
		},
		web: {
			port: 8082,
			bind_address: '127.0.0.1',
			ssl_enabled: false
		}
	});
	
	onMount(() => {
		loadConfiguration();
		loadAvailableModels();
	});
	
	async function loadConfiguration() {
		isLoading = true;
		statusMessage = 'Loading configuration...';
		statusType = 'info';

		try {
			const response = await fetch('/api/config', {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
				}
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}

			const data = await response.json();

			// Update config with real data from API
			config.ai.provider = data.ai.provider;
			config.ai.model = data.ai.model;
			config.ai.temperature = data.ai.temperature;
			config.ai.max_tokens = data.ai.max_tokens;
			config.ai.api_key_configured = data.ai.api_key_configured || false;

			config.execution.mode = data.execution.mode;
			config.execution.require_confirmation = data.execution.require_confirmation;
			config.execution.timeout = data.execution.timeout;

			config.web.port = data.web.port;
			config.web.bind_address = data.web.bind_address;
			config.web.ssl_enabled = data.web.ssl_enabled;

			statusMessage = 'Configuration loaded successfully';
			statusType = 'success';
			addAlert('success', 'Configuration loaded successfully');
		} catch (error) {
			console.error('Error loading configuration:', error);
			statusMessage = 'Failed to load configuration';
			statusType = 'error';
			addAlert('error', `Failed to load configuration: ${error instanceof Error ? error.message : String(error)}`);
		} finally {
			isLoading = false;
		}
	}
	
	async function saveConfiguration() {
		isLoading = true;
		statusMessage = 'Saving configuration...';
		statusType = 'info';

		try {
			// Save AI configuration
			const aiResponse = await fetch('/api/config', {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({
					section: 'ai',
					updates: {
						provider: config.ai.provider,
						model: config.ai.model,
						temperature: config.ai.temperature,
						max_tokens: config.ai.max_tokens
					},
					validate_only: false
				})
			});

			if (!aiResponse.ok) {
				throw new Error(`Failed to save AI configuration: ${aiResponse.status}`);
			}

			// Save execution configuration
			const execResponse = await fetch('/api/config', {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({
					section: 'execution',
					updates: {
						mode: config.execution.mode,
						require_confirmation: config.execution.require_confirmation,
						timeout: config.execution.timeout
					},
					validate_only: false
				})
			});

			if (!execResponse.ok) {
				throw new Error(`Failed to save execution configuration: ${execResponse.status}`);
			}

			// Save web configuration
			const webResponse = await fetch('/api/config', {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({
					section: 'web',
					updates: {
						port: config.web.port,
						bind_address: config.web.bind_address,
						ssl_enabled: config.web.ssl_enabled
					},
					validate_only: false
				})
			});

			if (!webResponse.ok) {
				throw new Error(`Failed to save web configuration: ${webResponse.status}`);
			}

			const webData = await webResponse.json();

			statusMessage = 'Configuration saved successfully';
			statusType = 'success';

			if (webData.restart_required) {
				addAlert('warning', 'Configuration saved. Server restart required for web settings to take effect.');
			} else {
				addAlert('success', 'Configuration saved successfully');
			}
		} catch (error) {
			console.error('Error saving configuration:', error);
			statusMessage = 'Failed to save configuration';
			statusType = 'error';
			addAlert('error', `Failed to save configuration: ${error instanceof Error ? error.message : String(error)}`);
		} finally {
			isLoading = false;
		}
	}
	
	function addAlert(type: string, message: string) {
		const alert = {
			id: Date.now().toString(),
			type,
			message
		};
		alerts = [...alerts, alert];
		
		// Auto-remove after 5 seconds
		setTimeout(() => {
			removeAlert(alert.id);
		}, 5000);
	}
	
	function removeAlert(id: string) {
		alerts = alerts.filter(alert => alert.id !== id);
	}

	async function loadAvailableModels() {
		try {
			// Load available models based on the current provider
			const models = getModelsForProvider(config.ai.provider);
			availableModels = models;
		} catch (error) {
			console.error('Error loading available models:', error);
			// Fallback to default models
			availableModels = getModelsForProvider(config.ai.provider);
		}
	}

	function getModelsForProvider(provider: string): Array<{id: string, name: string}> {
		switch (provider) {
			case 'ollama':
				return [
					{ id: 'gemma3:latest', name: 'Gemma 3 (Latest)' },
					{ id: 'llama3.1:8b', name: 'Llama 3.1 8B' },
					{ id: 'llama3.1:70b', name: 'Llama 3.1 70B' },
					{ id: 'codellama', name: 'Code Llama' },
					{ id: 'mistral', name: 'Mistral 7B' },
					{ id: 'mixtral:8x7b', name: 'Mixtral 8x7B' },
					{ id: 'qwen2.5', name: 'Qwen 2.5' },
					{ id: 'gemma2', name: 'Gemma 2' }
				];
			case 'openai':
				return [
					{ id: 'gpt-4o', name: 'GPT-4o (Latest)' },
					{ id: 'gpt-4o-mini', name: 'GPT-4o Mini' },
					{ id: 'gpt-4-turbo', name: 'GPT-4 Turbo' },
					{ id: 'gpt-4', name: 'GPT-4' },
					{ id: 'gpt-3.5-turbo', name: 'GPT-3.5 Turbo' }
				];
			case 'anthropic':
				return [
					{ id: 'claude-3-5-sonnet-20241022', name: 'Claude 3.5 Sonnet (Latest)' },
					{ id: 'claude-3-5-haiku-20241022', name: 'Claude 3.5 Haiku' },
					{ id: 'claude-3-opus-20240229', name: 'Claude 3 Opus' },
					{ id: 'claude-3-sonnet-20240229', name: 'Claude 3 Sonnet' },
					{ id: 'claude-3-haiku-20240307', name: 'Claude 3 Haiku' }
				];
			default:
				return [{ id: config.ai.model, name: config.ai.model }];
		}
	}

	// Update available models when provider changes
	$effect(() => {
		loadAvailableModels();
	});
</script>

<div class="h-full overflow-auto p-6 bg-white dark:bg-chatgpt-bg-primary">
	<div class="max-w-4xl mx-auto">
		<div class="mb-8">
			<h1 class="text-3xl font-semibold text-gray-900 dark:text-chatgpt-text-primary mb-2">⚙️ Settings</h1>
			<p class="text-gray-600 dark:text-chatgpt-text-secondary">Manage your TuxPilot system configuration</p>
		</div>

		<div class="mb-6 flex items-center justify-between">
			<div class="flex items-center gap-3">
				<div class="px-3 py-1 rounded-full text-sm {statusType === 'success' ? 'bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-200' : statusType === 'error' ? 'bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-200' : statusType === 'warning' ? 'bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200' : 'bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-200'}">
					{#if isLoading}
					<div class="w-4 h-4 border-2 border-transparent border-t-current rounded-full animate-spin"></div>
					{/if}
					<span>{statusMessage}</span>
				</div>
			</div>
			<div class="flex gap-3">
				<button
					onclick={loadConfiguration}
					disabled={isLoading}
					class="flex items-center gap-2 px-4 py-2 bg-gray-600 hover:bg-gray-700 disabled:bg-gray-400 dark:bg-chatgpt-bg-tertiary dark:hover:bg-chatgpt-border dark:disabled:bg-chatgpt-border text-white rounded-lg transition-colors disabled:cursor-not-allowed"
				>
					🔄 Reload
				</button>
				<button
					onclick={saveConfiguration}
					disabled={isLoading}
					class="flex items-center gap-2 px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-400 dark:bg-green-700 dark:hover:bg-green-600 dark:disabled:bg-chatgpt-border text-white rounded-lg transition-colors disabled:cursor-not-allowed"
				>
					💾 Save Changes
				</button>
			</div>
		</div>

		{#if alerts.length > 0}
			<div class="space-y-3 mb-6">
				{#each alerts as alert (alert.id)}
					<div class="flex items-center justify-between p-4 rounded-lg {alert.type === 'success' ? 'bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800' : alert.type === 'error' ? 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800' : alert.type === 'warning' ? 'bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800' : 'bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800'}">
						<span class="text-sm {alert.type === 'success' ? 'text-green-800 dark:text-green-200' : alert.type === 'error' ? 'text-red-800 dark:text-red-200' : alert.type === 'warning' ? 'text-yellow-800 dark:text-yellow-200' : 'text-blue-800 dark:text-blue-200'}">{alert.message}</span>
						<button
							onclick={() => removeAlert(alert.id)}
							class="text-gray-500 hover:text-gray-700 dark:text-chatgpt-text-muted dark:hover:text-chatgpt-text-secondary"
						>×</button>
					</div>
				{/each}
			</div>
		{/if}

		<div class="space-y-8">
			<!-- AI Configuration -->
			<div class="bg-white dark:bg-chatgpt-bg-secondary rounded-lg border border-gray-200 dark:border-chatgpt-border p-6">
				<h3 class="text-lg font-semibold text-gray-900 dark:text-chatgpt-text-primary mb-6">🤖 AI Configuration</h3>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="space-y-2">
						<label for="ai-provider" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Provider</label>
						<select
							id="ai-provider"
							bind:value={config.ai.provider}
							class="w-full px-3 py-2 border border-gray-300 dark:border-chatgpt-border rounded-lg bg-white dark:bg-chatgpt-bg-tertiary text-gray-900 dark:text-chatgpt-text-primary focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						>
							<option value="ollama">Ollama</option>
							<option value="openai">OpenAI</option>
							<option value="anthropic">Anthropic</option>
						</select>
					</div>
					<div class="space-y-2">
						<label for="ai-model" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Model</label>
						<select
							id="ai-model"
							bind:value={config.ai.model}
							class="w-full px-3 py-2 border border-gray-300 dark:border-chatgpt-border rounded-lg bg-white dark:bg-chatgpt-bg-tertiary text-gray-900 dark:text-chatgpt-text-primary focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						>
							{#each availableModels as model}
								<option value={model.id}>{model.name}</option>
							{/each}
						</select>
					</div>
					<div class="space-y-2">
						<label for="ai-temperature" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Temperature</label>
						<input
							id="ai-temperature"
							type="number"
							bind:value={config.ai.temperature}
							min="0"
							max="2"
							step="0.1"
							class="w-full px-3 py-2 border border-gray-300 dark:border-chatgpt-border rounded-lg bg-white dark:bg-chatgpt-bg-tertiary text-gray-900 dark:text-chatgpt-text-primary focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>
					<div class="space-y-2">
						<label for="ai-max-tokens" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Max Tokens</label>
						<input
							id="ai-max-tokens"
							type="number"
							bind:value={config.ai.max_tokens}
							min="1"
							max="8192"
							class="w-full px-3 py-2 border border-gray-300 dark:border-chatgpt-border rounded-lg bg-white dark:bg-chatgpt-bg-tertiary text-gray-900 dark:text-chatgpt-text-primary focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>
				</div>
			</div>

			<!-- Execution Settings -->
			<div class="bg-white dark:bg-chatgpt-bg-secondary rounded-lg border border-gray-200 dark:border-chatgpt-border p-6">
				<h3 class="text-lg font-semibold text-gray-900 dark:text-chatgpt-text-primary mb-6">⚙️ Execution Settings</h3>
				<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
					<div class="space-y-2">
						<label for="execution-mode" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Mode</label>
						<select
							id="execution-mode"
							bind:value={config.execution.mode}
							class="w-full px-3 py-2 border border-gray-300 dark:border-chatgpt-border rounded-lg bg-white dark:bg-chatgpt-bg-tertiary text-gray-900 dark:text-chatgpt-text-primary focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						>
							<option value="supervised">Supervised</option>
							<option value="semi-auto">Semi-Automatic</option>
							<option value="autonomous">Autonomous</option>
							<option value="read-only">Read-Only</option>
						</select>
					</div>
					<div class="space-y-2">
						<label for="execution-confirmation" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Require Confirmation</label>
						<div class="flex items-center">
							<input
								id="execution-confirmation"
								type="checkbox"
								bind:checked={config.execution.require_confirmation}
								class="w-4 h-4 text-blue-600 bg-gray-100 dark:bg-chatgpt-bg-tertiary border-gray-300 dark:border-chatgpt-border rounded focus:ring-blue-500 focus:ring-2"
							/>
							<label for="execution-confirmation" class="ml-2 text-sm text-gray-700 dark:text-chatgpt-text-secondary">Enable confirmation prompts</label>
						</div>
					</div>
					<div class="space-y-2">
						<label for="execution-timeout" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Timeout (seconds)</label>
						<input
							id="execution-timeout"
							type="number"
							bind:value={config.execution.timeout}
							min="1"
							max="300"
							class="w-full px-3 py-2 border border-gray-300 dark:border-chatgpt-border rounded-lg bg-white dark:bg-chatgpt-bg-tertiary text-gray-900 dark:text-chatgpt-text-primary focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>
				</div>
			</div>

			<!-- Web Server Configuration -->
			<div class="bg-white dark:bg-chatgpt-bg-secondary rounded-lg border border-gray-200 dark:border-chatgpt-border p-6">
				<h3 class="text-lg font-semibold text-gray-900 dark:text-chatgpt-text-primary mb-6">🌐 Web Server Configuration</h3>
				<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
					<div class="space-y-2">
						<label for="web-port" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Port</label>
						<input
							id="web-port"
							type="number"
							bind:value={config.web.port}
							min="1"
							max="65535"
							class="w-full px-3 py-2 border border-gray-300 dark:border-chatgpt-border rounded-lg bg-white dark:bg-chatgpt-bg-tertiary text-gray-900 dark:text-chatgpt-text-primary focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>
					<div class="space-y-2">
						<label for="web-bind-address" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">Bind Address</label>
						<select
							id="web-bind-address"
							bind:value={config.web.bind_address}
							class="w-full px-3 py-2 border border-gray-300 dark:border-chatgpt-border rounded-lg bg-white dark:bg-chatgpt-bg-tertiary text-gray-900 dark:text-chatgpt-text-primary focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						>
							<option value="127.0.0.1">127.0.0.1 (localhost)</option>
							<option value="0.0.0.0">0.0.0.0 (all interfaces)</option>
						</select>
					</div>
					<div class="space-y-2">
						<label for="web-ssl-enabled" class="block text-sm font-medium text-gray-700 dark:text-chatgpt-text-secondary">SSL Enabled</label>
						<div class="flex items-center">
							<input
								id="web-ssl-enabled"
								type="checkbox"
								bind:checked={config.web.ssl_enabled}
								class="w-4 h-4 text-blue-600 bg-gray-100 dark:bg-chatgpt-bg-tertiary border-gray-300 dark:border-chatgpt-border rounded focus:ring-blue-500 focus:ring-2"
							/>
							<label for="web-ssl-enabled" class="ml-2 text-sm text-gray-700 dark:text-chatgpt-text-secondary">Enable HTTPS/SSL</label>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

