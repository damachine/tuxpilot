<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';

	// Theme management
	let theme = $state('light');
	let sidebarOpen = $state(true);

	onMount(() => {
		// Load theme from localStorage
		const savedTheme = localStorage.getItem('theme') || 'light';
		theme = savedTheme;
		updateTheme();

		// Check if we're on mobile and close sidebar by default
		if (window.innerWidth < 768) {
			sidebarOpen = false;
		}
	});

	function updateTheme() {
		if (browser) {
			document.documentElement.classList.toggle('dark', theme === 'dark');
			localStorage.setItem('theme', theme);
		}
	}

	function toggleTheme() {
		theme = theme === 'light' ? 'dark' : 'light';
		updateTheme();
	}

	function toggleSidebar() {
		sidebarOpen = !sidebarOpen;
	}

	// Make theme toggle available globally
	if (browser) {
		(window as any).toggleTheme = toggleTheme;
	}
</script>

<div class="flex h-screen bg-white dark:bg-gray-950">
	<!-- Sidebar -->
	<Sidebar bind:open={sidebarOpen} />

	<!-- Main content -->
	<div class="flex-1 flex flex-col min-w-0">
		<!-- Header -->
		<Header
			{sidebarOpen}
			onToggleSidebar={toggleSidebar}
		/>

		<!-- Content area -->
		<main class="flex-1 overflow-hidden">
			<slot />
		</main>
	</div>
</div>
