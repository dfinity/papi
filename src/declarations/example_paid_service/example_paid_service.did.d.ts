import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type PaymentError = {
    'InsufficientFunds' : { 'needed' : bigint, 'available' : bigint }
  };
export type Result = { 'Ok' : string } |
  { 'Err' : PaymentError };
export interface _SERVICE {
  'cost_1000_cycles' : ActorMethod<[], Result>,
  'free' : ActorMethod<[], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
