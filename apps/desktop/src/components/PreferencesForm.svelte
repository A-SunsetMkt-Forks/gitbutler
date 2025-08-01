<script lang="ts">
	import ReduxResult from '$components/ReduxResult.svelte';
	import Section from '$components/Section.svelte';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { inject } from '@gitbutler/shared/context';
	import { SectionCard, Textbox, Toggle } from '@gitbutler/ui';

	const { projectId }: { projectId: string } = $props();
	const projectsService = inject(PROJECTS_SERVICE);
	const projectResult = $derived(projectsService.getProject(projectId));
</script>

<ReduxResult {projectId} result={projectResult.current}>
	{#snippet children(project)}
		<Section gap={8}>
			<SectionCard orientation="row" labelFor="omitCertificateCheck">
				{#snippet title()}
					Ignore host certificate checks
				{/snippet}
				{#snippet caption()}
					Enabling this will ignore host certificate checks when authenticating with ssh.
				{/snippet}
				{#snippet actions()}
					<Toggle
						id="omitCertificateCheck"
						checked={project.omit_certificate_check}
						onchange={async (value: boolean) => {
							await projectsService.updateProject({
								...project,
								omit_certificate_check: value
							});
						}}
					/>
				{/snippet}
			</SectionCard>

			<SectionCard orientation="row" centerAlign>
				{#snippet title()}
					Snapshot lines threshold
				{/snippet}
				{#snippet caption()}
					The number of lines that trigger a snapshot when saving.
				{/snippet}

				{#snippet actions()}
					<Textbox
						type="number"
						width={100}
						textAlign="center"
						value={project.snapshot_lines_threshold?.toString()}
						minVal={5}
						maxVal={1000}
						showCountActions
						onchange={async (value: string) => {
							await projectsService.updateProject({
								...project,
								snapshot_lines_threshold: parseInt(value)
							});
						}}
					/>
				{/snippet}
			</SectionCard>
		</Section>
	{/snippet}
</ReduxResult>
