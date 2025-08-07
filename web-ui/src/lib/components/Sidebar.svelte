<script lang="ts">
	import {
		IconMessageCircle,
		IconSettings,
		IconShield,
		IconPackage,
		IconNetwork,
		IconCpu,
		IconX
	} from '@tabler/icons-svelte';
	
	interface Props {
		open: boolean;
	}

	let { open = $bindable() }: Props = $props();
	
	const navigationItems = [
		{ id: 'chat', label: 'AI Chat', icon: IconMessageCircle, href: '/' },
		{ id: 'settings', label: 'Settings', icon: IconSettings, href: '/settings' }
	];
	
	const agentItems = [
		{ id: 'system', label: 'System Agent', icon: IconCpu, color: 'text-blue-600' },
		{ id: 'security', label: 'Security Agent', icon: IconShield, color: 'text-red-600' },
		{ id: 'package', label: 'Package Agent', icon: IconPackage, color: 'text-green-600' },
		{ id: 'network', label: 'Network Agent', icon: IconNetwork, color: 'text-purple-600' }
	];
	
	function closeSidebar() {
		open = false;
	}
</script>

<!-- Overlay for mobile -->
{#if open}
	<div 
		class="fixed inset-0 bg-black bg-opacity-50 z-40 lg:hidden"
		onclick={closeSidebar}
	></div>
{/if}

<!-- Sidebar -->
<aside
	class="fixed lg:relative inset-y-0 left-0 z-50 w-64 bg-white dark:bg-chatgpt-bg-secondary border-r border-gray-200 dark:border-chatgpt-border transform transition-transform duration-200 ease-in-out {open ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}"
>
	<div class="flex flex-col h-full">
		<!-- Header -->
		<div class="p-4 border-b border-gray-200 dark:border-chatgpt-border lg:hidden">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<div class="w-8 h-8 bg-primary-600 rounded-lg flex items-center justify-center">
						<span class="text-white font-bold text-lg">🐧</span>
					</div>
					<span class="font-semibold text-gray-900 dark:text-chatgpt-text-primary">TuxPilot</span>
				</div>
				<button
					onclick={closeSidebar}
					class="p-1 rounded-lg hover:bg-gray-100 dark:hover:bg-chatgpt-bg-tertiary"
				>
					<IconX size={20} class="text-gray-600 dark:text-chatgpt-text-secondary" />
				</button>
			</div>
		</div>
		
		<!-- Navigation -->
		<nav class="flex-1 p-4 space-y-6">
			<!-- Main navigation -->
			<div>
				<h3 class="text-xs font-semibold text-gray-500 dark:text-chatgpt-text-muted uppercase tracking-wider mb-3">
					Navigation
				</h3>
				<ul class="space-y-1">
					{#each navigationItems as item}
						<li>
							<a
								href={item.href}
								class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors text-gray-700 dark:text-chatgpt-text-secondary hover:bg-gray-100 dark:hover:bg-chatgpt-bg-tertiary"
							>
								<item.icon size={20} />
								<span class="font-medium">{item.label}</span>
							</a>
						</li>
					{/each}
				</ul>
			</div>
			
			<!-- AI Agents -->
			<div>
				<h3 class="text-xs font-semibold text-gray-500 dark:text-chatgpt-text-muted uppercase tracking-wider mb-3">
					AI Agents
				</h3>
				<ul class="space-y-1">
					{#each agentItems as agent}
						<li>
							<a
								href="/?agent={agent.id}"
								class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors text-gray-700 dark:text-chatgpt-text-secondary hover:bg-gray-100 dark:hover:bg-chatgpt-bg-tertiary"
							>
								<agent.icon size={20} class={agent.color} />
								<span class="text-sm">{agent.label}</span>
							</a>
						</li>
					{/each}
				</ul>
			</div>
		</nav>

		<!-- Footer -->
		<div class="p-4 border-t border-gray-200 dark:border-chatgpt-border">
			<div class="text-xs text-gray-500 dark:text-chatgpt-text-muted">
				<p>TuxPilot v0.1.0</p>
				<p>AI-powered Linux management</p>
			</div>
		</div>
	</div>
</aside>
