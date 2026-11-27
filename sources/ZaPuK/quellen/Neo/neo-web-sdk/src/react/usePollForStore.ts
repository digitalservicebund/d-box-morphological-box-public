// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef, useState } from 'react';
export function usePollForStore<T>(
	getStore: () => T | undefined,
	timeoutMs: number = 5000,
) {
	const initialStore = getStore();
	const [store, setStore] = useState<T | undefined>(initialStore);
	const [loading, setLoading] = useState(!initialStore);
	const [timeoutReached, setTimeoutReached] = useState(false);
	const getStoreRef = useRef(getStore);

	useEffect(() => {
		getStoreRef.current = getStore;
		let mounted = true;
		let finished = false;
		let pollInterval: ReturnType<typeof setInterval> | undefined = undefined;
		let timeout: ReturnType<typeof setTimeout> | undefined = undefined;

		// If store is already present, update state and skip polling.
		// This branch is hit when the effect re-runs because `getStore`'s identity
		// changed (new arrow function on every render) — the previous effect's
		// interval was cancelled by cleanup before it could call setLoading(false),
		// so we must do it here.
		const s = getStoreRef.current();
		if (s) {
			if (mounted) {
				setStore(s);
				setLoading(false);
			}
			return;
		}

		function poll() {
			if (finished) return;
			const s = getStoreRef.current();
			if (s && mounted) {
				setStore(s);
				setLoading(false);
				finished = true;
				if (pollInterval) clearInterval(pollInterval);
				if (timeout) clearTimeout(timeout);
			}
		}

		pollInterval = setInterval(poll, 500);
		timeout = setTimeout(() => {
			if (!finished) {
				setLoading(false);
				setTimeoutReached(true);
				finished = true;
				if (pollInterval) clearInterval(pollInterval);
			}
		}, timeoutMs);

		return () => {
			mounted = false;
			finished = true;
			if (pollInterval) clearInterval(pollInterval);
			if (timeout) clearTimeout(timeout);
		};
	}, [timeoutMs, getStore]);

	return { store, loading, timeoutReached };
}
