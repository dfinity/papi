export const idlFactory = ({ IDL }) => {
  const PaymentError = IDL.Variant({
    'InsufficientFunds' : IDL.Record({
      'needed' : IDL.Nat64,
      'available' : IDL.Nat64,
    }),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : PaymentError });
  return IDL.Service({
    'cost_1000_attached_cycles' : IDL.Func([], [Result], []),
    'free' : IDL.Func([], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
