<script lang="ts">
	import { IconMenu2, IconSun, IconMoon } from '@tabler/icons-svelte';
	
	interface Props {
		sidebarOpen: boolean;
		onToggleSidebar: () => void;
	}

	let { sidebarOpen, onToggleSidebar }: Props = $props();
	
	function toggleTheme() {
		if (typeof window !== 'undefined' && (window as any).toggleTheme) {
			(window as any).toggleTheme();
		}
	}
	

</script>

<header class="h-16 bg-white dark:bg-chatgpt-bg-secondary border-b border-gray-200 dark:border-chatgpt-border flex items-center justify-between px-4 lg:px-6">
	<!-- Left section -->
	<div class="flex items-center gap-4">
		<!-- Sidebar toggle -->
		<button
			onclick={onToggleSidebar}
			class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-chatgpt-bg-tertiary transition-colors lg:hidden"
			aria-label="Toggle sidebar"
		>
			<IconMenu2 size={20} class="text-gray-600 dark:text-chatgpt-text-secondary" />
		</button>
		
		<!-- Logo and title -->
		<div class="flex items-center gap-3">
			<div class="w-8 h-8 bg-primary-600 rounded-lg flex items-center justify-center">
				<span class="text-white font-bold text-lg">🐧</span>
			</div>
			<div class="hidden sm:block">
				<h1 class="text-lg font-semibold text-gray-900 dark:text-chatgpt-text-primary">TuxPilot</h1>
				<p class="text-xs text-gray-500 dark:text-chatgpt-text-muted">AI-Powered Linux Assistant</p>
			</div>
		</div>
	</div>
	
	<!-- Right section -->
	<div class="flex items-center gap-2">

		
		<!-- Theme toggle -->
		<button
			onclick={toggleTheme}
			class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-chatgpt-bg-tertiary transition-colors"
			aria-label="Toggle theme"
		>
			<IconSun size={20} class="text-gray-600 dark:text-chatgpt-text-secondary dark:hidden" />
			<IconMoon size={20} class="text-gray-600 dark:text-chatgpt-text-secondary hidden dark:block" />
		</button>
	</div>
</header>
