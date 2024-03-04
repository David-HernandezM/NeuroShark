import { ProgramMetadata } from '@gear-js/api';
import { useApi, useAlert, TemplateAlertOptions, useAccount } from '@gear-js/react-hooks';
import voucherUtils from './useVoucherUtils';
import { gasToSpend } from '../utils';
import { AnyJson, AnyNumber, Signer } from '@polkadot/types/types';
import { web3FromSource } from '@polkadot/extension-dapp';

const useContractUtils = () => {
    const { api } = useApi();
    const { account } = useAccount();
    const alert = useAlert();
    const { 
        voucherExists,
        voucherExpired,
        renewVoucherOneHour,
        accountVoucherId,
        addOneTokenToVoucher,
        voucherBalance,
        checkVoucherForUpdates
      } = voucherUtils();

    const sendMessageWithVoucher = async (
        userAddress: `0x${string}`,
        userMetaSource: string,
        programId: `0x${string}`,
        programMetadata: ProgramMetadata,
        payload: any, 
        value: number,
        successMessage: string,
        failMessage: string,
        loadingMessage: string,
        titleLoadingMessage: string,
    ): Promise<boolean> => {
        return new Promise(async (resolve, reject) => {
            if (!api) {
                alert.error("Api or accout is not started!!!");
                reject("Api is not started");
                return;
            }

            const voucherId = await accountVoucherId(programId, userAddress);

            await checkVoucherForUpdates(voucherId as `0x${string}`, userAddress);


            const totalGas = await api.program.calculateGas.handle(
                userAddress,
                programId,
                payload,
                value,
                false,
                programMetadata
            );

            console.log("Gas to spend: ", gasToSpend(totalGas));

            const { signer } = await web3FromSource(userMetaSource);

            const transferExtrinsic = api.message.send({
                destination: programId,
                payload,
                gasLimit: gasToSpend(totalGas),
                value,
                prepaid: true,
                account: userAddress
            }, programMetadata);

            const voucherTx = api.voucher.call(voucherId, { SendMessage: transferExtrinsic });

            try {
                await signMessageTransaction(
                    userAddress,
                    signer,
                    voucherTx,
                    successMessage,
                    failMessage,
                    loadingMessage,
                    titleLoadingMessage
                );

                resolve(true);
            } catch (e) {
                console.log("Error while sign transaction");
                reject("Error while sign transaction");
            }
            
        });
    }

    const sendMessage = async (
        userAddress: `0x${string}`,
        userMetaSource: string,
        programId: `0x${string}`,
        programMetadata: ProgramMetadata,
        payload: any, 
        value: AnyNumber,
        successMessage: string,
        failMessage: string,
        loadingMessage: string,
        titleLoadingMessage: string,
    ): Promise<boolean> => {
        return new Promise(async (resolve, reject) => {
            if (!api) {
                alert.error("Api es not started!");
                reject("Api is not started");
                return;
            }

            const totalGas = await api.program.calculateGas.handle(
                userAddress,
                programId,
                payload,
                value,
                false,
                programMetadata
            );

            console.log("Gas to spend: ", gasToSpend(totalGas));

            const { signer } = await web3FromSource(userMetaSource);

            const transferExtrinsic = api.message.send({
                destination: programId,
                payload,
                gasLimit: gasToSpend(totalGas),
                value,
                prepaid: false,
                account: userAddress
            }, programMetadata);

            try {
                await signMessageTransaction(
                    userAddress,
                    signer,
                    transferExtrinsic,
                    successMessage,
                    failMessage,
                    loadingMessage,
                    titleLoadingMessage
                )

                resolve(true);
            } catch (e) {
                console.log("Error while sign transaction");
                reject("Error while sign transaction");
            }
            
        });
    }

    const readState = async (programId: `0x${string}`, programMetadata: string, payload: any): Promise<AnyJson> => {
        return new Promise(async (resolve, reject) => {
            if (!api) {
                alert.error("Api es not started!");
                reject("Api is not started");
                return;
            }

            const contractState = await api
                .programState
                .read(
                    {
                        programId,
                        payload
                    },
                    ProgramMetadata.from(programMetadata)
                );
            
            resolve(contractState.toJSON());
        });
    };


    const signMessageTransaction = async (
        userAddress: `0x${string}`,
        signer: Signer,
        extrinsic: any,
        successMessage: string,
        failMessage: string,
        loadingMessage: string,
        titleLoadingMessage: string,
      ): Promise<boolean> => {
        return new Promise(async (resolve, reject) => {
    
          const alertOptions: TemplateAlertOptions = {
            title: titleLoadingMessage,
          };
    
          let loadingMessageId: any = null;
        
          try {
            await extrinsic
            .signAndSend(
              userAddress,
              { signer },
              ({ status, events }: { status: any, events: any }) => {
                if (!loadingMessageId) {
                    loadingMessageId = alert.loading(loadingMessage, alertOptions);
                }
                if (status.isInBlock) {
                  console.log(
                    `Completed at block hash #${status.asInBlock.toString()}`
                  );
                  alert.success(`Block hash #${status.asInBlock.toString()}`);
                } else {
                  console.log(`Current status: ${status.type}`);
                  if (status.type === "Finalized") {
                    alert.remove(loadingMessageId);
                    alert.success(successMessage);
                    resolve(true);
                  }
                }
              }
            )
          } catch(error: any) {
            console.log(":( transaction failed", error);
            if (loadingMessageId) alert.remove(loadingMessageId);
            reject("Error while sign transaction, or sending message");
          }
        });
      };

    return {
        sendMessageWithVoucher,
        sendMessage,
        signMessageTransaction,
        readState
    };
}

export default useContractUtils;