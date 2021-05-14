@0xe1cc1dc6c71d2d0e;

# Entity PartyRole(party.party): Party Role
struct PartyRole {
    # keys
    partyId @0 :Text;   # id
    roleTypeId @1 :Text;   # id
    # fields
}

# Entity Payment(accounting.payment): Payment
struct Payment {
    # keys
    paymentId @0 :Text;   # id
    # fields
    paymentTypeId @1 :Text;   # id
    paymentMethodTypeId @2 :Text;   # id
    paymentMethodId @3 :Text;   # id
    paymentGatewayResponseId @4 :Text;   # id
    paymentPreferenceId @5 :Text;   # id
    partyIdFrom @6 :Text;   # id
    partyIdTo @7 :Text;   # id
    roleTypeIdTo @8 :Text;   # id
    statusId @9 :Text;   # id
    effectiveDate @10 :Text;   # date-time
    paymentRefNum @11 :Text;   # short-varchar
    amount @12 :Float64;   # currency-amount
    currencyUomId @13 :Text;   # id
    comments @14 :Text;   # comment
    finAccountTransId @15 :Text;   # id
    overrideGlAccountId @16 :Text;   # id
    actualCurrencyAmount @17 :Float64;   # currency-amount
    actualCurrencyUomId @18 :Text;   # id
}
