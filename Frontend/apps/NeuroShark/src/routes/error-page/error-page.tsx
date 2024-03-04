import { Header } from "@/components"
import { useAccount } from "@gear-js/react-hooks"
import { NotFound } from "@/components/layout/not-found";

interface ErrorPageProps {
    isRootError?: boolean
}

export default function ErrorPage({ isRootError }: ErrorPageProps) {
    const { isAccountReady } = useAccount();
    return isRootError 
    ? <>
        <Header isAccountVisible={isAccountReady} />
        <NotFound />
      </>
    : <NotFound />
}