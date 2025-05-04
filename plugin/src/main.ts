import {
	App,
	ButtonComponent,
	Editor,
	MarkdownFileInfo,
	MarkdownView,
	Modal, Notice,
	Plugin,
	PluginSettingTab,
	Setting,
	TFile
} from 'obsidian';

interface RecursiveShareSettings {
	serverUri: string;
	apiKey: string;
}

const DEFAULT_SETTINGS: RecursiveShareSettings = {
	serverUri: '',
	apiKey: '',
}

type FileToUpload = {
	name: string;
} & (
	{ tfile: TFile; }
	| { value: string; }
	)

type ShareMetadata = {
	id: string;
	title: string;
	created_at: string;
}

async function createShare(config: RecursiveShareSettings, name: string): Promise<[string, string]> {
	let share: ShareMetadata = await fetch(`${config.serverUri}/api/create`, {
		headers: {
			"Content-Type": "application/json",
			"Authorization": config.apiKey
		},
		method: "POST",
		body: JSON.stringify({
			root_name: name
		})
	}).then(res => res.json())

	let hyphenated_title = (share.title.split("/").last() || "").replace(/[^a-zA-Z0-9]/g, "-")
	let link = `${config.serverUri}/view/${share.id}#-${hyphenated_title}`

	return [link, share.id];
}

async function uploadFiles(files: FileToUpload[], config: RecursiveShareSettings, id: string) {
	for (const file of files) {
		let fileContent: any;
		if ('tfile' in file) {
			fileContent = await this.app.vault.readBinary(file.tfile);
		} else {
			fileContent = file.value;
		}
		const formData = new FormData();
		const blob = new Blob([fileContent]);
		formData.append('file', blob, file.name);
		formData.append('name', file.name);

		const response = await fetch(`${config.serverUri}/api/${id}`, {
			method: 'PUT',
			headers: {
				"Authorization": config.apiKey
			},
			body: formData
		});

		if (!response.ok) {
			throw new Error(`Failed to upload file ${file.name}: ${response.statusText}`);
		}

		new Notice("Upload of " + file.name + " complete.")
	}

	new Notice("--- Upload complete ---")
}

export default class RecursiveSharePlugin extends Plugin {
	settings: RecursiveShareSettings;

	async onload() {
		await this.loadSettings();

		this.addCommand({
			id: 'share',
			name: 'Share recursively',
			editorCallback: (editor: Editor, ctx: MarkdownView | MarkdownFileInfo) => {
				let view = ctx as MarkdownView;
				let info = ctx as MarkdownFileInfo;
				let name = info.file!.name
				let content = editor.getValue()
				let file = view.file!;
				let cache = this.app.metadataCache.getFileCache(file)!

				let resourcesToUpload: FileToUpload[] = []

				if (cache.embeds) {
					for (const embed of cache.embeds) {
						let destFile = this.app.metadataCache.getFirstLinkpathDest(embed.link, file.path);
						if (!destFile) continue;
						let destName = embed.link;
						while (destName.startsWith("../") || destName.startsWith("..\\")) {
							destName = destName.substring(3);
						}
						if (destName.startsWith("/") || destName.startsWith("\\")) {
							destName = destName.substring(1);
						}
						resourcesToUpload.push({
							name: destName,
							tfile: destFile
						})

						let urlencoded = encodeURIComponent(destName);
						content = content.replace(embed.original, `![${embed.displayText}](${this.settings.serverUri}/api/##id##/${urlencoded})`)
					}
				}

				createShare(this.settings, name)
					.then(([link, id]) => {
						resourcesToUpload.push({
							name: name,
							value: content.replace(/##id##/g, id)
						})
						uploadFiles(resourcesToUpload, this.settings, id)
						return link;
					})
					.then(share => {
						new ShareLinkModal(this.app, share).open();
					})
			}
		});

		// This adds a settings tab so the user can configure various aspects of the plugin
		this.addSettingTab(new RecursiveShareSettingsTab(this.app, this));
	}

	onunload() {

	}

	async loadSettings() {
		this.settings = Object.assign({}, DEFAULT_SETTINGS, await this.loadData());
	}

	async saveSettings() {
		await this.saveData(this.settings);
	}
}

class RecursiveShareSettingsTab extends PluginSettingTab {
	plugin: RecursiveSharePlugin;

	constructor(app: App, plugin: RecursiveSharePlugin) {
		super(app, plugin);
		this.plugin = plugin;
	}

	display(): void {
		const {containerEl} = this;

		containerEl.empty();

		containerEl.createEl('h2', {text: 'Recursive Share Settings'});
		containerEl.createEl('a', {
			text: 'Click here for a guide on how to host an instance',
			href: 'https://github.com/Bloeckchengrafik/obsidian-recursive-share'
		});

		new Setting(containerEl)
			.setName('Root Server URL')
			.setDesc('You\'ll need a hosted instance')
			.addText(text => text
				.setPlaceholder('Enter your URI')
				.setValue(this.plugin.settings.serverUri)
				.onChange(async (value) => {
					this.plugin.settings.serverUri = value;
					await this.plugin.saveSettings();
				}));

		new Setting(containerEl)
			.setName("API Key")
			.setDesc("See the linked guide for more information")
			.addText(text => text
				.setPlaceholder("Enter your API key")
				.setValue(this.plugin.settings.apiKey)
				.onChange(async (value) => {
					this.plugin.settings.apiKey = value;
					await this.plugin.saveSettings();
				}));
	}
}


export class ShareLinkModal extends Modal {
	linkToShare: string;

	constructor(app: App, linkToShare: string) {
		super(app);
		this.linkToShare = linkToShare;
	}

	onOpen() {
		const {contentEl} = this;

		// Add the main text
		contentEl.createEl('p', {text: "Here's your share link:"});

		// Add an input field to display the link and make it easy to select/copy
		const linkInput = contentEl.createEl('input', {
			type: 'text',
			value: this.linkToShare,
			cls: 'share-link-input' // Optional: Add a class for potential styling
		});
		linkInput.setAttr('readonly', 'true'); // Make it read-only so user doesn't accidentally edit

		// Optional: Add some basic styling for the input field
		linkInput.style.width = '100%';
		linkInput.style.padding = '5px';
		linkInput.style.marginTop = '10px';
		linkInput.style.marginBottom = '10px';

		// Add a button to copy the link
		new ButtonComponent(contentEl)
			.setButtonText("Copy Link")
			.setCta() // Apply call-to-action styling
			.onClick(() => {
				navigator.clipboard.writeText(this.linkToShare).then(() => {
					// Optionally, provide feedback to the user
					this.close(); // Close the modal after copying
					// You could also add a confirmation message instead of closing immediately
				}).catch(err => {
					console.error('Failed to copy link: ', err);
					// Optionally, show an error message to the user
				});
			});
	}

	onClose() {
		const {contentEl} = this;
		contentEl.empty(); // Clean up the modal content
	}
}
