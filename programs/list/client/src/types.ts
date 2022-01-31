import { AccountData } from "@chronos-so/utils";
import { ListProgram } from "./idl";

export type ElementAccountData = AccountData<
  ListProgram["accounts"][0],
  ListProgram
>;

export type ListAccountData = AccountData<
  ListProgram["accounts"][1],
  ListProgram
>;
