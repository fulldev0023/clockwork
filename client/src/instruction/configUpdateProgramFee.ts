import { BN, Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type ConfigUpdateProgramFeeArgs = {
  newProgramFee: BN;
};

export class ConfigUpdateProgramFee {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async configUpdateProgramFee({
    newProgramFee,
  }: ConfigUpdateProgramFeeArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const configData = await this.account.config.data(configPDA.address);
    return this.cronos.instruction.configUpdateProgramFee(newProgramFee, {
      accounts: {
        admin: configData.admin,
        config: configPDA.address,
      },
    });
  }
}
