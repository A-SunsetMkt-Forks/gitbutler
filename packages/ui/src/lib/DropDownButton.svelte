<script lang="ts">
	import ContextMenu from '$lib/ContextMenu.svelte';
	import Button from '@gitbutler/ui/Button.svelte';
	import Tooltip from '@gitbutler/ui/Tooltip.svelte';
	import type iconsJson from '@gitbutler/ui/data/icons.json';
	import type { ComponentColorType, ComponentKindType } from '@gitbutler/ui/utils/colorTypes';
	import type { Snippet } from 'svelte';

	interface Props {
		testId?: string;
		icon?: keyof typeof iconsJson;
		style?: ComponentColorType;
		kind?: ComponentKindType;
		disabled?: boolean;
		loading?: boolean;
		wide?: boolean;
		grow?: boolean;
		width?: number | string;
		autoClose?: boolean;
		tooltip?: string;
		type?: 'button' | 'submit' | 'reset';
		menuPosition?: 'top' | 'bottom';
		children: Snippet;
		contextMenuSlot: Snippet;
		onclick?: (e: MouseEvent) => void;
	}

	const {
		testId,
		icon,
		style = 'neutral',
		kind = 'solid',
		disabled = false,
		loading = false,
		wide = false,
		grow = false,
		width,
		autoClose = false,
		type,
		tooltip,
		menuPosition = 'bottom',
		children,
		contextMenuSlot,
		onclick
	}: Props = $props();

	let contextMenu = $state<ReturnType<typeof ContextMenu>>();
	let iconEl = $state<HTMLElement>();
	let visible = $state(false);

	function preventContextMenu(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
	}

	export function show() {
		visible = true;
		contextMenu?.open();
	}

	export function close() {
		visible = false;
		contextMenu?.close();
	}
</script>

<Tooltip text={tooltip}>
	<div class="dropdown-wrapper" class:wide class:grow>
		<div class="dropdown">
			<Button
				{testId}
				{style}
				{icon}
				{kind}
				{type}
				{width}
				reversedDirection
				disabled={disabled || loading}
				dropdownChild
				{onclick}
				oncontextmenu={preventContextMenu}
			>
				{@render children()}
			</Button>
			<Button
				bind:el={iconEl}
				{style}
				{kind}
				icon={visible ? 'chevron-up' : 'chevron-down'}
				{loading}
				disabled={disabled || loading}
				dropdownChild
				onclick={() => {
					visible = !visible;
					contextMenu?.toggle();
				}}
				oncontextmenu={preventContextMenu}
			/>
		</div>
		<ContextMenu
			bind:this={contextMenu}
			leftClickTrigger={iconEl}
			verticalAlign={menuPosition}
			onclose={() => {
				visible = false;
			}}
			onclick={() => {
				if (autoClose) {
					contextMenu?.close();
				}
			}}
			onkeypress={() => {
				if (autoClose) {
					contextMenu?.close();
				}
			}}
		>
			{@render contextMenuSlot()}
		</ContextMenu>
	</div>
</Tooltip>

<style lang="postcss">
	.dropdown-wrapper {
		/* display set directly on element */
		position: relative;
	}

	.grow {
		flex-grow: 1;
	}

	.dropdown {
		display: flex;
		flex-grow: 1;
		align-items: center;
	}
	.wide {
		width: 100%;
	}
</style>
