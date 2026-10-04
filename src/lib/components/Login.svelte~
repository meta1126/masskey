<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { open } from "@tauri-apps/plugin-shell";

	let instanceUrl = $state("");
	let code = $state("");
	let step = $state("input" as "input" | "code");
	let errorMsg = $state("");
	let isLoggedIn = $state(false);
	let instanceInfo = $state(
		null as {
			uri: string;
			title: string;
			description: string;
			email: string;
			users_count: number;
		} | null,
	);
	let loadingTimeline = $state(false);
	let timelineError = $state("");

	// Timeline type selection
	type TimelineType = "home" | "public" | "tagged";
	let timelineType = $state("home" as TimelineType);
	let hashtag = $state("");

	interface TimelineStatus {
		id: string;
		account: {
			username: string;
			display_name: string;
			avatar: string;
		};
		content: string;
		created_at: string;
		reblogs_count: number;
		favourites_count: number;
	}

	let timeline = $state([] as TimelineStatus[]);
	let hasFetched = $state(false);

	$effect(() => {
		if (isLoggedIn && !hasFetched) {
			hasFetched = true;
			fetchTimeline();
		}
	});

	async function fetchTimeline() {
		loadingTimeline = true;
		timelineError = "";
		timeline = [];
		try {
			if (timelineType === "home") {
				timeline = await invoke<TimelineStatus[]>("get_home_timeline", {
					limit: 20,
				});
			} else if (timelineType === "public") {
				timeline = await invoke<TimelineStatus[]>("get_public_timeline", {
					local: false,
					limit: 20,
				});
			} else if (timelineType === "tagged") {
				if (!hashtag.trim()) {
					timelineError = "ハッシュタグを入力してください";
					loadingTimeline = false;
					return;
				}
				timeline = await invoke<TimelineStatus[]>("get_tagged_timeline", {
					tag: hashtag.trim().replace(/^#/, ""),
					local: false,
					limit: 20,
				});
			}
		} catch (e) {
			timelineError = String(e);
		} finally {
			loadingTimeline = false;
		}
	}

	async function handleStart() {
		errorMsg = "";
		timelineError = "";
		try {
			const url = await invoke<string>("start_login", { instanceUrl });
			await open(url);
			step = "code";
		} catch (e) {
			errorMsg = String(e);
			step = "input";
		}
	}

	async function handleComplete() {
		errorMsg = "";
		timeline = [];
		timelineError = "";
		hasFetched = false;
		try {
			const account = await invoke("complete_login", { code });
			isLoggedIn = true;
			try {
				instanceInfo = await invoke("get_instance_info");
			} catch (e) {
				instanceInfo = {
					uri: account.instance_url,
					title: "",
					description: "",
					email: "",
					users_count: 0,
				};
			}
		} catch (e) {
			errorMsg = String(e);
		}
	}

	function handleLogout() {
		isLoggedIn = false;
		instanceInfo = null;
		timeline = [];
		timelineError = "";
		errorMsg = "";
		step = "input";
		instanceUrl = "";
		code = "";
		timelineType = "home";
		hashtag = "";
		hasFetched = false;
	}

	function stripTags(html: string): string {
		return html.replace(/<[^>]*>/g, "");
	}

	function getTimelineTitle(): string {
		switch (timelineType) {
			case "home":
				return "ホームタイムライン";
			case "public":
				return "パブリックタイムライン";
			case "tagged":
				return `#${hashtag} のタイムライン`;
		}
	}
</script>

<div
	style="margin-top: 2rem; padding: 1rem; border: 1px solid #ccc; border-radius: 8px;"
>
	<h2>Mastodon ログイン</h2>

	{#if !isLoggedIn}
		{#if step === "input"}
			<div style="margin-bottom: 1rem;">
				<label>インスタンスURL:</label><br />
				<input
					bind:value={instanceUrl}
					placeholder="https://mastodon.social"
					style="width: 100%; padding: 0.5rem; margin-top: 0.25rem;"
				/>
			</div>
			<button onclick={handleStart}>ログイン開始</button>
		{:else}
			<p>ブラウザで表示された認可コードを貼り付けてください</p>
			<div style="margin-bottom: 1rem;">
				<label>認可コード:</label><br />
				<input
					bind:value={code}
					placeholder="認可コード"
					style="width: 100%; padding: 0.5rem; margin-top: 0.25rem;"
				/>
			</div>
			<button onclick={handleComplete}>ログイン完了</button>
		{/if}
	{:else}
		<!-- Logged in state -->
		<div
			style="margin-bottom: 1rem; padding: 0.75rem; background: #f0f0f0; border-radius: 4px;"
		>
			<p>
				<strong>ログイン済み:</strong>
				{instanceInfo?.title || instanceInfo?.uri}
			</p>
			{#if instanceInfo?.description}
				<p style="font-size: 0.9em; color: #555;">{instanceInfo.description}</p>
			{/if}
		</div>

		<!-- Timeline type selection -->
		<div style="margin-bottom: 1rem;">
			<label style="margin-right: 0.5rem;">タイムライン:</label>
			<select bind:value={timelineType} style="padding: 0.25rem;">
				<option value="home">ホーム</option>
				<option value="public">パブリック</option>
				<option value="tagged">ハッシュタグ</option>
			</select>
		</div>

		{#if timelineType === "tagged"}
			<div style="margin-bottom: 1rem;">
				<label>ハッシュタグ:</label><br />
				<input
					bind:value={hashtag}
					placeholder="例: rust ( # は不要 )"
					style="width: 100%; padding: 0.5rem; margin-top: 0.25rem;"
				/>
			</div>
		{/if}

		<div style="margin-bottom: 1rem;">
			<button onclick={fetchTimeline} disabled={loadingTimeline}>
				{#if loadingTimeline}読み込み中...{:else}{getTimelineTitle()} を取得{/if}
			</button>
			<button
				onclick={handleLogout}
				style="margin-left: 0.5rem; background: #ff4444; color: white;"
				>ログアウト</button
			>
		</div>

		{#if timelineError}
			<p style="color: red; margin-bottom: 1rem;">{timelineError}</p>
		{/if}

		{#if timeline.length > 0}
			<div style="margin-top: 1rem;">
				<h3>{getTimelineTitle()} ({timeline.length} 件)</h3>
				{#each timeline as status}
					<div style="padding: 1rem; border-bottom: 1px solid #eee;">
						<div
							style="display: flex; align-items: flex-start; margin-bottom: 0.5rem;"
						>
							{#if status.account.avatar}
								<img
									src={status.account.avatar}
									alt=""
									style="width: 48px; height: 48px; border-radius: 50%; margin-right: 0.75rem; flex-shrink: 0;"
								/>
							{:else}
								<div
									style="width: 48px; height: 48px; border-radius: 50%; background: #ddd; margin-right: 0.75rem; flex-shrink: 0; display: flex; align-items: center; justify-content: center; font-size: 1.2em;"
								>
									?
								</div>
							{/if}
							<div>
								<strong
									>{status.account.display_name ||
										status.account.username}</strong
								>
								<span style="color: #666; font-size: 0.9em;">
									@{status.account.username}</span
								>
								<span
									style="color: #999; font-size: 0.8em; margin-left: 0.5rem;"
									>{status.created_at}</span
								>
							</div>
						</div>
						<div
							style="white-space: pre-wrap; word-break: break-word; margin-bottom: 0.5rem;"
						>
							{stripTags(status.content)}
						</div>
						<div style="color: #888; font-size: 0.85em;">
							<span>🔄 {status.reblogs_count}</span>
							<span style="margin-left: 1rem;"
								>❤️ {status.favourites_count}</span
							>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{/if}

	{#if errorMsg}
		<p style="color: red; margin-top: 1rem;">{errorMsg}</p>
	{/if}
</div>
