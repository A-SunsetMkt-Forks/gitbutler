import type { AnyCommit } from '$lib/commits/commit';
import type { CommitData, Author } from '@gitbutler/ui/commitLines/types';

export function transformAnyCommit(anyCommit: AnyCommit): CommitData {
	const output = pullCommitDetails(anyCommit);

	if (anyCommit.relatedTo) {
		output.relatedRemoteCommit = pullCommitDetails(anyCommit.relatedTo);
	}

	return output;
}

function pullCommitDetails(anyCommit: AnyCommit): CommitData {
	const author: Author = {
		name: anyCommit.author.name,
		email: anyCommit.author.email,
		gravatarUrl: anyCommit.author.gravatarUrl
	};

	return {
		id: anyCommit.id,
		title: anyCommit.descriptionTitle,
		author
	};
}
