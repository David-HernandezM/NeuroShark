import { Account, useAccount, useApi } from "@gear-js/react-hooks";
import { Outlet, useOutletContext } from "react-router-dom";
import { Header } from "@/components";
import { useState } from "react";
import { GearApi } from "@gear-js/api";
import { ApiLoader } from '@/components';

export type ContextType = { 
	gearApi: GearApi | undefined,
	polkadotAccount: Account | undefined
};

export default function Root() {
    const { isApiReady } = useApi();
	const { isAccountReady } = useAccount();

	const isAppReady = isApiReady && isAccountReady;

    return (
		<>
			<Header isAccountVisible={isAccountReady} />
			<div>
				{
					isAppReady
					? <Outlet/>
					: <ApiLoader />
				}
			</div>
		</>
	)
}

export function useGearHooks() {
	return useOutletContext<ContextType>();
}