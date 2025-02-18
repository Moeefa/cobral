import { useCallback, useEffect, useState } from "react";

import { check, type Update } from "@tauri-apps/plugin-updater";
import { listen } from "@tauri-apps/api/event";
import { toast } from "@/hooks/use-toast";

export const useUpdater = () => {
	const [update, setUpdate] = useState<Update | null>(null);
	const [open, setOpen] = useState(false);

	const checkForAppUpdates = useCallback(async () => {
		toast({
			title: "Procurando atualizações...",
		});

		try {
			const update = await check();

			if (update?.available) {
				setOpen(true);
				setUpdate(update);
				return true;
			}

			return false;
		} catch (e) {
			console.error(e);
			toast({
				title: "Não foi possível verificar atualizações",
			});
			return false;
		}
	}, []);

	const checkForAppUpdatesListener = useCallback(async () => {
		const unlisten = await listen("check-updates", async () => {
			console.log("Checking for updates...");

			if (!(await checkForAppUpdates())) {
				toast({
					title: "Nenhuma atualização disponível",
				});
			}
		});
		return () => {
			unlisten();
		};
	}, [checkForAppUpdates]);

	useEffect(() => {
		checkForAppUpdates();
	}, [checkForAppUpdates]);

	useEffect(() => {
		checkForAppUpdatesListener();
	}, [checkForAppUpdatesListener]);

	return {
		checkForAppUpdates,
		checkForAppUpdatesListener,
		open,
		setOpen,
		update,
	};
};
