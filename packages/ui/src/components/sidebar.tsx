import { useContext, useState } from "react";
import { basename, join } from "@tauri-apps/api/path";
import {
	type DirEntry,
	readDir,
	readFile,
	writeFile,
	mkdir,
	remove,
	create,
	rename,
} from "@tauri-apps/plugin-fs";
import { open, save } from "@tauri-apps/plugin-dialog";
import { EllipsisIcon } from "lucide-react";
import { Tree, File, Folder } from "@/components/ui/file-tree";
import { EditorContext, type EditorFile } from "@/contexts/editor-context";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from "@/components/ui/tooltip";
import { platform } from "@tauri-apps/plugin-os";
import { Link } from "react-router-dom";
import {
	ContextMenu,
	ContextMenuContent,
	ContextMenuItem,
	ContextMenuLabel,
	ContextMenuSeparator,
	ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { Input } from "@/components/ui/input";
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from "@/components/ui/dialog";
import { useToast } from "@/hooks/use-toast";
import { useFileWatcher } from "@/hooks/use-filewatcher";
import { Button } from "@/components/ui/button";

export const FileSystemUtils = {
	async processEntriesRecursively(
		parent: string,
		entries: DirEntry[],
	): Promise<EditorFile[]> {
		const files: EditorFile[] = [];

		for (const entry of entries) {
			const currentPath = await join(parent, entry.name);

			if (entry.isFile && entry.name.endsWith(".cl")) {
				const content = await readFile(currentPath);
				files.push({
					id: currentPath,
					isSelectable: true,
					name: entry.name,
					path: currentPath,
					content: new TextDecoder().decode(content),
					children: [],
				});
			}

			if (entry.isDirectory) {
				const subFiles = await this.processEntriesRecursively(
					currentPath,
					await readDir(currentPath),
				);

				files.push({
					id: currentPath,
					isSelectable: false,
					name: entry.name,
					path: currentPath,
					children: subFiles,
					isDirectory: true,
				});
			}
		}

		return files;
	},

	async findFile(
		files: EditorFile[],
		path: string,
	): Promise<EditorFile | undefined> {
		for (const file of files) {
			if (file.path === path) {
				return file;
			}
			if (file.children) {
				const found = await this.findFile(file.children, path);
				if (found) return found;
			}
		}
		return undefined;
	},
};

const FileOperations = () => {
	const { value, setValue, setCurrentFile, setFiles, setAbsolutePath } =
		useContext(EditorContext);

	const handleSaveFile = async () => {
		try {
			const file = await save({
				filters: [{ name: "arquivo", extensions: ["cl"] }],
			});

			if (!file) return;

			const encoder = new TextEncoder();
			const data = encoder.encode(value);
			await writeFile(file, data, { create: true });
		} catch (error) {
			console.error("Error saving file:", error);
		}
	};

	const handleOpenFile = async () => {
		try {
			const file = await open({
				multiple: false,
				directory: false,
				canCreateDirectories: true,
				filters: [{ name: "arquivo", extensions: ["cl"] }],
			});

			if (!file) return;

			const content = await readFile(file);
			const fileName = await basename(file);
			const decodedContent = new TextDecoder().decode(content);

			const newFile: EditorFile = {
				id: file,
				isSelectable: true,
				name: fileName,
				path: file,
				content: decodedContent,
				children: [],
			};

			setValue(decodedContent);
			setFiles([newFile]);
			setCurrentFile(newFile);
		} catch (error) {
			console.error("Error opening file:", error);
		}
	};

	const handleOpenDirectory = async () => {
		try {
			const dir = await open({
				multiple: false,
				directory: true,
				canCreateDirectories: true,
			});

			if (!dir) return;

			setAbsolutePath(dir);
			const entries = await readDir(dir);
			const files = await FileSystemUtils.processEntriesRecursively(
				dir,
				entries,
			);
			setFiles(files);

			if (files.length > 0 && files[0].content) {
				setValue(files[0].content);
				setCurrentFile(files[0]);
			}
		} catch (error) {
			console.error("Error opening directory:", error);
		}
	};

	return (
		<div className="flex items-center px-4">
			<TooltipProvider>
				<DropdownMenu>
					<Tooltip>
						<TooltipTrigger asChild>
							<DropdownMenuTrigger className="flex items-center justify-center">
								<EllipsisIcon className="size-4" />
							</DropdownMenuTrigger>
						</TooltipTrigger>
						<TooltipContent>Ações</TooltipContent>
					</Tooltip>
					<DropdownMenuContent className="w-24">
						<DropdownMenuItem onClick={handleSaveFile}>
							Salvar arquivo
						</DropdownMenuItem>
						<DropdownMenuItem onClick={handleOpenFile}>
							Abrir arquivo
						</DropdownMenuItem>
						<DropdownMenuItem onClick={handleOpenDirectory}>
							Abrir pasta
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</TooltipProvider>
		</div>
	);
};

interface TreeNodeProps {
	node: EditorFile;
	parentPath: string;
	onFileSelect: (file: EditorFile, path: string) => Promise<void>;
}

type ShowDialogProps = (
	type: "newFile" | "newFolder" | "rename" | "delete",
	item?: EditorFile,
) => void;

const TreeNode = ({
	node,
	parentPath,
	showDialog,
	onFileSelect,
}: TreeNodeProps & {
	showDialog: ShowDialogProps;
}) => {
	const currentPath = parentPath ? `${parentPath}\\${node.name}` : node.name;
	const { currentFile, absolutePath } = useContext(EditorContext);

	if (!node.children || (node.children.length === 0 && !node.isDirectory)) {
		return (
			<ContextMenu key={node.id}>
				<ContextMenuTrigger>
					<Link to="/">
						<File
							key={node.id}
							name={node.name}
							value={node.id}
							handleSelect={() => onFileSelect(node, currentPath)}
							isSelect={currentFile?.id === node.id}
						>
							<p className="select-none text-nowrap">{node.name}</p>
						</File>
					</Link>
				</ContextMenuTrigger>
				<ContextMenuContent className="w-48">
					<ContextMenuLabel>{node.name}</ContextMenuLabel>
					<ContextMenuSeparator />
					{absolutePath ? (
						<>
							<ContextMenuItem onClick={() => showDialog("newFile")}>
								Criar novo arquivo
							</ContextMenuItem>
							<ContextMenuItem onClick={() => showDialog("newFolder")}>
								Criar nova pasta
							</ContextMenuItem>
							<ContextMenuItem onClick={() => showDialog("delete", node)}>
								Deletar
							</ContextMenuItem>
						</>
					) : (
						<></>
					)}
					<ContextMenuItem onClick={() => showDialog("rename", node)}>
						Renomear
					</ContextMenuItem>
				</ContextMenuContent>
			</ContextMenu>
		);
	}

	return (
		<ContextMenu key={node.id}>
			<ContextMenuTrigger>
				<Folder key={node.id} element={node.name} value={node.id}>
					{node.children
						.sort((a, b) =>
							a.isDirectory === b.isDirectory
								? a.name.localeCompare(b.name)
								: a.isDirectory
									? -1
									: 1,
						)
						.map((child) => (
							<TreeNode
								key={child.id}
								node={child}
								parentPath={currentPath}
								onFileSelect={onFileSelect}
								showDialog={showDialog}
							/>
						))}
				</Folder>
			</ContextMenuTrigger>
			<ContextMenuContent className="w-48">
				<ContextMenuLabel>{node.name}</ContextMenuLabel>
				<ContextMenuSeparator />
				<ContextMenuItem onClick={() => showDialog("newFile")}>
					Criar novo arquivo
				</ContextMenuItem>
				<ContextMenuItem onClick={() => showDialog("newFolder")}>
					Criar nova pasta
				</ContextMenuItem>
				<ContextMenuItem onClick={() => showDialog("delete", node)}>
					Deletar
				</ContextMenuItem>
				<ContextMenuItem onClick={() => showDialog("rename", node)}>
					Renomear
				</ContextMenuItem>
			</ContextMenuContent>
		</ContextMenu>
	);
};

const FileTreeView = ({
	showDialog,
}: {
	showDialog: ShowDialogProps;
}) => {
	const { files, setValue, setCurrentFile, absolutePath } =
		useContext(EditorContext);

	const handleFileSelect = async (file: EditorFile, _path: string) => {
		if (!file.content && file.path) {
			try {
				const content = await readFile(file.path);
				file.content = new TextDecoder().decode(content);
			} catch (error) {
				console.error("Error reading file:", error);
				return;
			}
		}

		setValue(file.content || "");
		setCurrentFile(file);
	};

	const getFolderIds = (nodes: EditorFile[]): string[] => {
		return nodes.reduce((acc: string[], node) => {
			if (node.children && node.children.length > 0) {
				acc.push(node.id);
				acc.push(
					...getFolderIds(node.children.map((child) => child as EditorFile)),
				);
			}
			return acc;
		}, []);
	};

	if (!files.length) return <></>;

	if (!absolutePath)
		return (
			<Tree
				initialExpandedItems={getFolderIds(files)}
				className="p-2 h-full overflow-hidden"
				elements={files}
			>
				{files
					.sort((a, b) =>
						a.isDirectory === b.isDirectory
							? a.name.localeCompare(b.name)
							: a.isDirectory
								? -1
								: 1,
					)
					.map((file) => (
						<TreeNode
							key={file.id}
							node={file}
							parentPath=""
							onFileSelect={handleFileSelect}
							showDialog={showDialog}
						/>
					))}
			</Tree>
		);

	return (
		<ContextMenu>
			<ContextMenuTrigger>
				<Tree
					initialExpandedItems={getFolderIds(files)}
					className="p-2 h-full overflow-hidden"
					elements={files}
				>
					{files
						.sort((a, b) =>
							a.isDirectory === b.isDirectory
								? a.name.localeCompare(b.name)
								: a.isDirectory
									? -1
									: 1,
						)
						.map((file) => (
							<TreeNode
								key={file.id}
								node={file}
								parentPath=""
								onFileSelect={handleFileSelect}
								showDialog={showDialog}
							/>
						))}
				</Tree>
			</ContextMenuTrigger>
			<ContextMenuContent className="w-48">
				<ContextMenuItem onClick={() => showDialog("newFile")}>
					Criar novo arquivo
				</ContextMenuItem>
				<ContextMenuItem onClick={() => showDialog("newFolder")}>
					Criar nova pasta
				</ContextMenuItem>
			</ContextMenuContent>
		</ContextMenu>
	);
};

interface FileDialogState {
	isOpen: boolean;
	type: "newFile" | "newFolder" | "rename" | "delete";
	item: EditorFile | null;
	path: string;
}

export const Sidebar = () => {
	const currentPlatform = platform();
	const { toast } = useToast();
	const { absolutePath, setCurrentFile } = useContext(EditorContext);
	const { refreshFileTree } = useFileWatcher();
	const [dialogState, setDialogState] = useState<FileDialogState>({
		isOpen: false,
		type: "newFile",
		item: null,
		path: "",
	});
	const [inputValue, setInputValue] = useState("");

	const handleSubmit = async () => {
		const { type, item, path } = dialogState;
		const basePath = item?.path || absolutePath;

		try {
			switch (type) {
				case "newFile": {
					try {
						const filePath = await join(basePath || path, inputValue);
						await create(filePath);
					} catch (error) {
						toast({
							title: "Erro ao criar arquivo",
							description: String(error),
							variant: "destructive",
						});
					}
					break;
				}
				case "newFolder": {
					try {
						const folderPath = await join(basePath || path, inputValue);
						await mkdir(folderPath);
					} catch (error) {
						toast({
							title: "Erro ao criar pasta",
							description: String(error),
							variant: "destructive",
						});
					}
					break;
				}
				case "rename": {
					try {
						if (!item?.path) return;
						const parentPath = item.path.slice(0, item.path.lastIndexOf("\\"));
						const newPath = await join(parentPath, inputValue);
						await rename(item.path, newPath);
					} catch (error) {
						toast({
							title: "Erro ao renomear",
							description: String(error),
							variant: "destructive",
						});
					}
					break;
				}
				case "delete": {
					try {
						if (!item?.path) return;
						await remove(item.path, { recursive: true });
						if (item.isSelectable) {
							setCurrentFile(null);
						}
						await refreshFileTree();
					} catch (error) {
						toast({
							title: "Erro ao deletar",
							description: String(error),
							variant: "destructive",
						});
					}
					break;
				}
			}

			refreshFileTree();
			setDialogState((prev) => ({ ...prev, isOpen: false }));
			setInputValue("");
		} catch (error) {
			console.error("Error:", error);
		}
	};

	const showDialog: ShowDialogProps = (type, item) => {
		setDialogState({
			isOpen: true,
			type,
			item: item || null,
			path: item?.path || "",
		});
		setInputValue(item?.name || "");
	};

	return (
		<div className="h-full bg-white/85 dark:bg-neutral-900/70">
			<div className="flex flex-col h-full [&>span]:h-full">
				<div
					data-tauri-drag-region
					className="flex items-center justify-between w-full border-b border-border min-h-11 p-[3.5px]"
				>
					{currentPlatform !== "macos" && (
						<p
							data-tauri-drag-region
							className="text-sm hidden sm:block select-none font-medium ml-3.5"
						>
							Explorador
						</p>
					)}
					<FileOperations />
				</div>
				<FileTreeView showDialog={showDialog} />
			</div>

			<Dialog
				open={dialogState.isOpen}
				onOpenChange={(open) =>
					setDialogState((prev) => ({ ...prev, isOpen: open }))
				}
			>
				<DialogContent>
					<DialogHeader>
						<DialogTitle>
							{dialogState.type === "newFile"
								? "Novo arquivo"
								: dialogState.type === "newFolder"
									? "Nova pasta"
									: dialogState.type === "rename"
										? "Renomear"
										: "Deletar"}
						</DialogTitle>
					</DialogHeader>
					<div className="grid gap-4 py-4">
						{dialogState.type !== "delete" && (
							<Input
								value={inputValue}
								onChange={(e) => setInputValue(e.target.value)}
								onKeyDown={(e) => {
									if (e.key === "Enter") handleSubmit();
								}}
								placeholder={
									dialogState.type === "newFile" ? "filename.cl" : "name"
								}
							/>
						)}
						<div className="flex justify-end gap-2">
							<Button onClick={handleSubmit}>
								{dialogState.type === "delete" ? "Deletar" : "Confirmar"}
							</Button>
							<Button
								variant="destructive"
								onClick={() =>
									setDialogState((prev) => ({ ...prev, isOpen: false }))
								}
							>
								Cancelar
							</Button>
						</div>
					</div>
				</DialogContent>
			</Dialog>
		</div>
	);
};
