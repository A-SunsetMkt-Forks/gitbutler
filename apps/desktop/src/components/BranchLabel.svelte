<script lang="ts">
	import { autoSelectBranchNameFeature } from '$lib/config/uiFeatureFlags';
	import { TestId } from '$lib/testing/testIds';
	import { resizeObserver } from '@gitbutler/ui/utils/resizeObserver';

	interface Props {
		name: string;
		disabled?: boolean;
		readonly?: boolean;
		fontSize?: '14' | '15';
		allowClear?: boolean;
		onChange?: (value: string) => void;
		onDblClick?: () => void;
	}

	const {
		name,
		disabled = false,
		fontSize = '14',
		readonly = false,
		allowClear,
		onChange,
		onDblClick
	}: Props = $props();

	let inputEl: HTMLInputElement | undefined = $state();
	let editableName = $derived(name);
	let nameWidth = $state(0);
	let editableNameWidth = $state(0);
	const nameWidthPx = $derived(`${Math.max(nameWidth, editableNameWidth)}px`);
</script>

<span
	data-testid={TestId.BranchNameLabel}
	use:resizeObserver={(e) => {
		nameWidth = Math.round(e.frame.width);
	}}
	class="branch-name-measure-el text-{fontSize} text-bold"
>
	{name}
</span>

<span
	use:resizeObserver={(e) => {
		editableNameWidth = Math.round(e.frame.width);
	}}
	class="branch-name-measure-el text-{fontSize} text-bold"
>
	{editableName}
</span>
<input
	type="text"
	{disabled}
	{readonly}
	bind:this={inputEl}
	bind:value={editableName}
	onchange={(e) => {
		const value = e.currentTarget.value.trim();
		if (value === name) return;
		if (value === '' && !allowClear) {
			editableName = name;
			return;
		}
		onChange?.(value);
	}}
	title={editableName}
	class="branch-name-input text-{fontSize} text-bold"
	ondblclick={(e) => {
		e.stopPropagation();
		onDblClick?.();
	}}
	oncontextmenu={(e) => {
		e.stopPropagation();
	}}
	onclick={(e) => {
		if (readonly) return;
		e.stopPropagation();
		inputEl?.focus();
		if ($autoSelectBranchNameFeature) {
			inputEl?.select();
		}
	}}
	onkeypress={(e) => {
		if (readonly) return;
		e.stopPropagation();
	}}
	onfocus={() => {
		editableName = name;
	}}
	onkeydown={(e) => {
		if (e.key === 'Enter' || e.key === 'Escape' || e.key === 'Tab') {
			inputEl?.blur();
		}
	}}
	autocomplete="off"
	autocorrect="off"
	spellcheck="false"
	style:width={nameWidthPx}
/>

<style lang="postcss">
	.branch-name-measure-el,
	.branch-name-input {
		min-width: 44px;
		height: 20px;
		padding: 2px 3px;
		border: 1px solid transparent;
	}
	.branch-name-measure-el {
		display: inline-block;
		visibility: hidden;
		visibility: hidden;
		position: fixed;
		width: fit-content;
		border: 2px solid transparent;
		color: black;
		white-space: pre;
		pointer-events: none;
	}
	.branch-name-input {
		width: 100%;

		max-width: 100%;
		overflow: hidden;
		border-radius: var(--radius-s);
		outline: none;
		background-color: transparent;
		color: var(--clr-text-1);
		text-overflow: ellipsis;
		white-space: nowrap;
		transition:
			border var(--transition-fast),
			background-color var(--transition-fast);

		/* not readonly */
		&:not([readonly]):not([disabled]):not(:focus):hover {
			border: 1px solid var(--clr-border-2);
		}

		&:not([readonly]):not([disabled]):focus {
			border: 1px solid var(--clr-border-2);
			outline: none;
			background-color: var(--clr-bg-1-muted);
		}
	}
	.branch-name-input[readonly] {
		cursor: default;
	}
</style>
