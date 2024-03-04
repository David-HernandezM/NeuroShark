import { GearKeyring, IUpdateVoucherParams } from '@gear-js/api';
import { useAccount, useApi, useAlert, TemplateAlertOptions, useBalanceFormat } from '@gear-js/react-hooks';
import { ONE_TVARA_VALUE, seed, VOUCHER_MIN_LIMIT } from '../consts';

const useVoucherUtils = () => {
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { api } = useApi();
  const { account } = useAccount();
  const alert = useAlert();

  const checkVoucherForUpdates = async (voucherId: `0x${string}`, account: `0x${string}`): Promise<boolean> => {
    return new Promise(async (resolve, reject) => {
      const voucherAlreadyCreated = await voucherExists(voucherId, account);
      if (voucherAlreadyCreated) {
        reject("Voucher does not exists");
        return;
      }

      if (await voucherExpired(voucherId)) {
        await renewVoucherOneHour(voucherId, account);
      }

      const totalVoucherBalance = await voucherBalance(voucherId);

      if (totalVoucherBalance < VOUCHER_MIN_LIMIT) {
        await addOneTokenToVoucher(voucherId);
      }

      resolve(true);
    });
  }

  const generateNewVoucher = (program_id: `0x${string}`): Promise<string> => {
    return new Promise(async (resolve, reject) => {
      if (!api || !account) {
        console.log('Api or account does not started');
        reject('Error creating new voucher');
        return;
      }

      const voucherIssued = await api.voucher.issue(
        account?.decodedAddress ?? '0x00',
        ONE_TVARA_VALUE * 11, // 11 TVaras
        api.voucher.minDuration, // 1_200, // An hour in blocks
        [program_id],
      );

      try {
        await signTransaction(
            voucherIssued.extrinsic, 
            "Voucher created",
            "Error creating voucher",
            "Creating voucher",
            "NeuroShar:"
          );
        resolve(voucherIssued.voucherId);
      } catch (e) {
        console.log("Error processing transaction");
        reject("Error while sign transaction");
      }

      

    //   const alertOptions: TemplateAlertOptions = {
    //     title: 'NeuroShark:',
    //   };

    //   const creatingVoucherAlertId = alert.loading('Creating voucher', alertOptions);

    //   const keyring = await GearKeyring.fromSeed(seed, 'AdminDavid');

    //   try {
    //     await voucherIssued.extrinsic.signAndSend(keyring, async (event) => {
    //       console.log(event.toHuman());
    //       const extrinsicJSON: any = event.toHuman();
    //       if (extrinsicJSON && extrinsicJSON.status !== 'Ready') {
    //         const objectKey = Object.keys(extrinsicJSON.status)[0];
    //         if (objectKey === 'Finalized') {
    //           alert.remove(creatingVoucherAlertId);
    //           alert.success('Voucher created');
    //           resolve(voucherIssued.voucherId);
    //         }
    //       }
    //     });
    //   } catch (error: any) {
    //     console.error(`${error.name}: ${error.message}`);
    //     alert.remove(creatingVoucherAlertId);
    //     alert.error('Error creating voucher');
    //     reject('Error creating voucher');
    //   }
    });
  };

  const voucherExpired = async (voucherId: string): Promise<boolean> => {
    return new Promise(async (resolve, reject) => {
      if (!api || !account) {
        console.log('Api or Account is not ready');
        reject(false);
        return;
      }

      const voucherData = await api.voucher.getDetails(account.decodedAddress, voucherId);
      const blockHash = await api.blocks.getFinalizedHead();
      const blocks = await api.blocks.getBlockNumber(blockHash as Uint8Array);

      resolve(blocks.toNumber() > voucherData.expiry);
    });
  };

  const voucherBalance = async (voucherId: string): Promise<number> => {
    return new Promise(async (resolve, reject) => {
      if (!api) {
        console.log('api or account is not ready');
        reject(false);
        return;
      }

      const voucherBalance = await api.balance.findOut(voucherId);
      const voucherBalanceFormated = Number(getFormattedBalanceValue(voucherBalance.toString()).toFixed());

      resolve(voucherBalanceFormated);
    });
  };

  const voucherExists = async (program_id: `0x${string}`, account: `0x${string}`): Promise<boolean> => {
    return new Promise(async (resolve, reject) => {
      if (!api) {
        console.log('api is not ready');
        reject(false);
        return;
      }

      const vouchers = await api.voucher.getAllForAccount(account, program_id);

      resolve(Object.keys(vouchers).length > 0);
    });
  };

  const accountVoucherId = async (program_id: `0x${string}`, account: `0x${string}`): Promise<string> => {
    return new Promise(async (resolve, reject) => {
      if (!api) {
        console.log('api or account is not ready');
        reject(false);
        return;
      }

      const vouchersData = await api.voucher.getAllForAccount(account, program_id);
      const vouchersId = Object.keys(vouchersData);

      if (vouchersId.length < 1) {
        console.log('User does not has voucher');
        reject(false);
        return;
      }

      resolve(vouchersId[0]);
    });
  };

  const renewVoucherOneHour = async (voucherId: string, account: `0x${string}`): Promise<boolean> => {
    return new Promise(async (resolve, reject) => {
      if (!api) {
        console.log('Api or Account is not ready');
        reject(false);
        return;
      }

      const newVoucherData: IUpdateVoucherParams = {
        prolongDuration: 1_200, // one hour
      };

      const voucherUpdate = api.voucher.update(account, voucherId, newVoucherData);

      try {
        await signTransaction(
            voucherUpdate,
            "Voucher updated",
            "Error renewing voucher",
            "Renewing voucher",
            "NeuroShark: "
        );
      } catch (e) {
        console.log("Error during sign transaction");
        reject("Error while sign transaction");
      }

    //   const renewVoucherAlertId = alert.loading('Renewing voucher', alertOptions);

    //   const alertOptions: TemplateAlertOptions = {
    //     title: 'Rutzo action',
    //   };

    //   const keyring = await GearKeyring.fromSeed(seed, 'AdminDavid');

    //   try {
    //     await voucherUpdate.signAndSend(keyring, async (event) => {
    //       console.log(event.toHuman());
    //       const extrinsicJSON: any = event.toHuman();
    //       if (extrinsicJSON && extrinsicJSON.status !== 'Ready') {
    //         const objectKey = Object.keys(extrinsicJSON.status)[0];
    //         if (objectKey === 'Finalized') {
    //           alert.remove(renewVoucherAlertId);
    //           alert.success('Voucher updated');
    //           console.log('Voucher updated');
    //           resolve(true);
    //         }
    //       }
    //     });
    //   } catch (error: any) {
    //     alert.remove(renewVoucherAlertId);
    //     alert.error('Error renewing voucher');
    //     console.error(`${error.name}: ${error.message}`);
    //     reject(false);
    //   }
    });
  };

  const addOneTokenToVoucher = async (voucherId: string): Promise<boolean> => {
    return new Promise(async (resolve, reject) => {
      if (!api || !account) {
        console.log('Api or Account is not ready');
        reject(false);
        return;
      }

      const newVoucherData: IUpdateVoucherParams = {
        balanceTopUp: ONE_TVARA_VALUE * 2,
      };

      const voucherUpdate = api.voucher.update(account.decodedAddress, voucherId, newVoucherData);

      try {
        await signTransaction(
            voucherUpdate,
            "Voucher updated",
            "Error while adding tokens to voucher",
            "Adding tokens to voucher",
            "NeuroShark"
        )
      } catch (e) {
        console.log("Error while sign transaction");
        reject("Error while sign transaction");
      }

    //   const alertOptions: TemplateAlertOptions = {
    //     title: 'Rutzo action',
    //   };

    //   const renewVoucherAlertId = alert.loading('Adding tokens to voucher', alertOptions);

    //   const keyring = await GearKeyring.fromSeed(seed, 'AdminDavid');

    //   try {
    //     await voucherUpdate.signAndSend(keyring, async (event) => {
    //       console.log(event.toHuman());
    //       const extrinsicJSON: any = event.toHuman();
    //       if (extrinsicJSON && extrinsicJSON.status !== 'Ready') {
    //         const objectKey = Object.keys(extrinsicJSON.status)[0];
    //         if (objectKey === 'Finalized') {
    //           alert.remove(renewVoucherAlertId);
    //           alert.success('Voucher updated');
    //           console.log('Voucher updated');
    //           resolve(true);
    //         }
    //       }
    //     });
    //   } catch (error: any) {
    //     alert.remove(renewVoucherAlertId);
    //     alert.error('Error adding tokens to voucher');
    //     console.error(`${error.name}: ${error.message}`);
    //     reject(false);
    //   }
    });
  };

  const signTransaction = async (
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

      let loadingMessageId = alert.loading(loadingMessage, alertOptions);
      const keyring = await GearKeyring.fromSeed(seed, 'AdminDavid');

      try {
        await extrinsic.signAndSend(keyring, async (event: any) => {
          console.log(event.toHuman());
          const extrinsicJSON: any = event.toHuman();
          if (extrinsicJSON && extrinsicJSON.status !== 'Ready') {
            const objectKey = Object.keys(extrinsicJSON.status)[0];
            if (objectKey === 'Finalized') {
              alert.remove(loadingMessageId);
              alert.success(successMessage);
              console.log(successMessage);
              resolve(true);
            }
          }
        });
      } catch (error: any) {
        alert.remove(loadingMessage);
        alert.error(failMessage);
        console.error(`${error.name}: ${error.message}`);
        reject(false);
      }
    });
  };

  return {
    generateNewVoucher,
    voucherExpired,
    voucherBalance,
    voucherExists,
    renewVoucherOneHour,
    accountVoucherId,
    addOneTokenToVoucher,
    signTransaction,
    checkVoucherForUpdates
  };
};

export default useVoucherUtils;
