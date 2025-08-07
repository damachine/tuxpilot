<script lang="ts">
	import { onMount } from 'svelte';
	import { IconSend, IconUser, IconRobot, IconCopy, IconCheck } from '@tabler/icons-svelte';
	import MessageContent from './MessageContent.svelte';
	
	interface Message {
		id: string;
		role: 'user' | 'assistant';
		content: string;
		timestamp: Date;
		agent?: string;
	}
	
	let messages: Message[] = $state([]);
	let inputText = $state('');
	let isLoading = $state(false);
	let messagesContainer: HTMLElement;
	
	// Example prompts for new users
	const examplePrompts = [
		"Check my system status and performance",
		"Scan for security vulnerabilities", 
		"Update all packages safely",
		"Diagnose network connectivity issues",
		"Clean up disk space and optimize performance",
		"Show me running services and their status"
	];
	
	onMount(() => {
		// Load conversation history if available
		loadConversationHistory();
	});
	
	function loadConversationHistory() {
		// TODO: Load from TuxPilot backend
		const savedMessages = localStorage.getItem('tuxpilot-messages');
		if (savedMessages) {
			messages = JSON.parse(savedMessages);
		}
	}
	
	function saveConversationHistory() {
		localStorage.setItem('tuxpilot-messages', JSON.stringify(messages));
	}
	
	async function sendMessage() {
		if (!inputText.trim() || isLoading) return;
		
		const userMessage: Message = {
			id: crypto.randomUUID(),
			role: 'user',
			content: inputText.trim(),
			timestamp: new Date()
		};
		
		messages = [...messages, userMessage];
		const currentInput = inputText;
		inputText = '';
		isLoading = true;
		
		try {
			// Send to TuxPilot backend
			const response = await fetch('/api/chat', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({
					message: currentInput,
					chat_id: 'main-session'
				})
			});
			
			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			
			const data = await response.json();
			
			const assistantMessage: Message = {
				id: data.message_id || crypto.randomUUID(),
				role: 'assistant',
				content: data.response || 'Sorry, I encountered an error processing your request.',
				timestamp: new Date(data.timestamp || Date.now()),
				agent: data.agent || 'system'
			};
			
			messages = [...messages, assistantMessage];
			saveConversationHistory();
			
		} catch (error) {
			console.error('Error sending message:', error);
			
			const errorMessage: Message = {
				id: crypto.randomUUID(),
				role: 'assistant',
				content: 'Sorry, I encountered an error connecting to the TuxPilot backend. Please check that the server is running.',
				timestamp: new Date()
			};
			
			messages = [...messages, errorMessage];
		} finally {
			isLoading = false;
			scrollToBottom();
		}
	}
	
	function scrollToBottom() {
		if (messagesContainer) {
			messagesContainer.scrollTop = messagesContainer.scrollHeight;
		}
	}
	
	function useExamplePrompt(prompt: string) {
		inputText = prompt;
	}
	
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			sendMessage();
		}
	}
	
	function clearConversation() {
		messages = [];
		localStorage.removeItem('tuxpilot-messages');
	}
</script>

<div class="flex flex-col h-full bg-white dark:bg-chatgpt-bg-primary">
	<!-- Messages area -->
	<div 
		bind:this={messagesContainer}
		class="flex-1 overflow-y-auto scrollbar-thin p-4 space-y-6"
	>
		{#if messages.length === 0}
			<!-- Welcome screen -->
			<div class="max-w-2xl mx-auto text-center py-12">
				<div class="w-16 h-16 bg-primary-600 rounded-full flex items-center justify-center mx-auto mb-6">
					<span class="text-white text-2xl">🐧</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-chatgpt-text-primary mb-4">
					Welcome to TuxPilot
				</h1>
				<p class="text-lg text-gray-600 dark:text-chatgpt-text-secondary mb-8">
					Your intelligent Linux system assistant. Ask me anything about system management, security, or performance optimization.
				</p>

				<!-- Example prompts -->
				<div class="grid grid-cols-1 md:grid-cols-2 gap-3 max-w-4xl mx-auto">
					{#each examplePrompts as prompt}
						<button
							onclick={() => useExamplePrompt(prompt)}
							class="p-4 text-left bg-gray-50 dark:bg-chatgpt-bg-secondary hover:bg-gray-100 dark:hover:bg-chatgpt-bg-tertiary rounded-lg border border-gray-200 dark:border-chatgpt-border transition-colors"
						>
							<p class="text-sm text-gray-700 dark:text-chatgpt-text-secondary">{prompt}</p>
						</button>
					{/each}
				</div>
			</div>
		{:else}
			<!-- Messages -->
			{#each messages as message}
				<div class="max-w-4xl mx-auto">
					<div class="flex gap-4 {message.role === 'user' ? 'justify-end' : 'justify-start'}">
						{#if message.role === 'assistant'}
							<div class="w-8 h-8 bg-primary-600 rounded-full flex items-center justify-center flex-shrink-0">
								<IconRobot size={16} class="text-white" />
							</div>
						{/if}
						
						<div class="flex-1 max-w-3xl">
							<div class="flex items-center gap-2 mb-2">
								<span class="text-sm font-medium text-gray-900 dark:text-chatgpt-text-primary">
									{message.role === 'user' ? 'You' : 'TuxPilot'}
								</span>
								{#if message.agent}
									<span class="text-xs px-2 py-1 bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 rounded-full">
										{message.agent}
									</span>
								{/if}
								<span class="text-xs text-gray-500 dark:text-chatgpt-text-muted">
									{message.timestamp.toLocaleTimeString()}
								</span>
							</div>

							<div class="bg-{message.role === 'user' ? 'primary-600 text-white' : 'gray-100 dark:bg-chatgpt-bg-secondary text-gray-900 dark:text-chatgpt-text-primary'} rounded-lg p-4">
								<MessageContent content={message.content} />
							</div>
						</div>
						
						{#if message.role === 'user'}
							<div class="w-8 h-8 bg-gray-600 rounded-full flex items-center justify-center flex-shrink-0">
								<IconUser size={16} class="text-white" />
							</div>
						{/if}
					</div>
				</div>
			{/each}
			
			<!-- Loading indicator -->
			{#if isLoading}
				<div class="max-w-4xl mx-auto">
					<div class="flex gap-4">
						<div class="w-8 h-8 bg-primary-600 rounded-full flex items-center justify-center flex-shrink-0">
							<IconRobot size={16} class="text-white" />
						</div>
						<div class="flex-1 max-w-3xl">
							<div class="bg-gray-100 dark:bg-chatgpt-bg-secondary rounded-lg p-4">
								<div class="flex items-center gap-2">
									<div class="w-2 h-2 bg-gray-400 dark:bg-chatgpt-text-muted rounded-full animate-pulse"></div>
									<div class="w-2 h-2 bg-gray-400 dark:bg-chatgpt-text-muted rounded-full animate-pulse" style="animation-delay: 0.2s"></div>
									<div class="w-2 h-2 bg-gray-400 dark:bg-chatgpt-text-muted rounded-full animate-pulse" style="animation-delay: 0.4s"></div>
									<span class="text-sm text-gray-600 dark:text-chatgpt-text-secondary ml-2">TuxPilot is thinking...</span>
								</div>
							</div>
						</div>
					</div>
				</div>
			{/if}
		{/if}
	</div>
	
	<!-- Input area -->
	<div class="border-t border-gray-200 dark:border-chatgpt-border p-4">
		<div class="max-w-4xl mx-auto">
			<div class="flex gap-3">
				<div class="flex-1">
					<textarea
						bind:value={inputText}
						onkeydown={handleKeydown}
						placeholder="Ask TuxPilot about your Linux system..."
						class="w-full px-4 py-3 border border-gray-300 dark:border-chatgpt-border rounded-lg resize-none focus:ring-2 focus:ring-primary-500 focus:border-transparent bg-white dark:bg-chatgpt-bg-secondary text-gray-900 dark:text-chatgpt-text-primary placeholder-gray-500 dark:placeholder-chatgpt-text-muted"
						rows="1"
						disabled={isLoading}
					></textarea>
				</div>
				<button
					onclick={sendMessage}
					disabled={!inputText.trim() || isLoading}
					class="px-4 py-3 bg-primary-600 hover:bg-primary-700 disabled:bg-gray-300 dark:disabled:bg-chatgpt-bg-tertiary text-white rounded-lg transition-colors disabled:cursor-not-allowed"
				>
					<IconSend size={20} />
				</button>
			</div>

			{#if messages.length > 0}
				<div class="flex justify-between items-center mt-3">
					<p class="text-xs text-gray-500 dark:text-chatgpt-text-muted">
						Press Enter to send, Shift+Enter for new line
					</p>
					<button
						onclick={clearConversation}
						class="text-xs text-gray-500 dark:text-chatgpt-text-muted hover:text-gray-700 dark:hover:text-chatgpt-text-secondary"
					>
						Clear conversation
					</button>
				</div>
			{/if}
		</div>
	</div>
</div>
