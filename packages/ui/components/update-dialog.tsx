import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from "@/components/ui/alert-dialog";

import { relaunch } from "@tauri-apps/plugin-process";
import { useUpdater } from "@/hooks/use-updater";

export function UpdateDialog() {
	const { open, setOpen, update } = useUpdater();

	return (
		<AlertDialog open={open}>
			<AlertDialogContent>
				<AlertDialogHeader>
					<AlertDialogTitle>Atualização</AlertDialogTitle>
					<AlertDialogDescription>
						Uma nova atualização está disponível. Deseja baixar e instalar
						agora? O aplicativo será reiniciado. Certifique-se de salvar seu
						trabalho.
					</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel onClick={() => setOpen(!open)}>
						Cancel
					</AlertDialogCancel>
					<AlertDialogAction
						onClick={async () => {
							if (update) {
								setOpen(false);
								await update.downloadAndInstall();
								await relaunch();
							}
						}}
					>
						Continue
					</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	);
}
