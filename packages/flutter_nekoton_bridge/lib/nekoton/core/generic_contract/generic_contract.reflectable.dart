// This file has been generated by the reflectable package.
// https://github.com/dart-lang/reflectable.

import 'dart:core';
import 'generic_contract.dart' as prefix1;
import 'package:flutter_nekoton_bridge/example_related/caller_test_class_wrapper.dart'
    as prefix2;
import 'package:flutter_nekoton_bridge/example_related/caller_wrapper.dart'
    as prefix3;
import 'package:flutter_nekoton_bridge/nekoton/core/token_wallet/token_wallet.dart'
    as prefix8;
import 'package:flutter_nekoton_bridge/nekoton/core/ton_wallet/ton_wallet.dart'
    as prefix9;
import 'package:flutter_nekoton_bridge/nekoton/external/gql_connection.dart'
    as prefix4;
import 'package:flutter_nekoton_bridge/nekoton/external/jrpc_connection.dart'
    as prefix5;
import 'package:flutter_nekoton_bridge/nekoton/external/ledger_connection.dart'
    as prefix6;
import 'package:flutter_nekoton_bridge/nekoton/external/storage.dart'
    as prefix7;
import 'package:flutter_nekoton_bridge/rust_to_dart/reflector.dart' as prefix0;

// ignore_for_file: camel_case_types
// ignore_for_file: implementation_imports
// ignore_for_file: prefer_adjacent_string_concatenation
// ignore_for_file: prefer_collection_literals
// ignore_for_file: unnecessary_const

// ignore:unused_import
import 'package:reflectable/mirrors.dart' as m;
// ignore:unused_import
import 'package:reflectable/src/reflectable_builder_based.dart' as r;
// ignore:unused_import
import 'package:reflectable/reflectable.dart' as r show Reflectable;

final _data = <r.Reflectable, r.ReflectorData>{
  const prefix0.Reflector(): r.ReflectorData(
      <m.TypeMirror>[
        r.NonGenericClassMirrorImpl(
            r'GenericContract',
            r'.GenericContract',
            134217735,
            0,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {r'subscribe': () => prefix1.GenericContract.subscribe},
            {},
            {},
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'getContractState': 0,
              r'getPendingTransactions': 0,
              r'getPollingMethod': 0,
              r'executeTransactionLocally': 1,
              r'estimateFees': 1,
              r'send': 1,
              r'refresh': 0,
              r'preloadTransactions': 2,
              r'handleBlock': 3,
              r'onMessageSent': 4,
              r'onMessageExpired': 4,
              r'onStateChanged': 4,
              r'onTransactionsFound': 4,
              r'initializeMirror': 0,
              r'contract': 0,
              r'contract=': 4,
              r'transport': 0,
              r'address': 0,
              r'address=': 4,
              r'contractState': 0,
              r'pendingTransactions': 0,
              r'pollingMethod': 0,
              r'fieldUpdatesStream': 0,
              r'onMessageSentStream': 0,
              r'onMessageExpiredStream': 0,
              r'onStateChangedStream': 0,
              r'onTransactionsFoundStream': 0,
              r'subscribe': 0
            }),
        r.NonGenericClassMirrorImpl(
            r'CallerTestClassWrapper',
            r'.CallerTestClassWrapper',
            134217735,
            1,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {r'create': () => prefix2.CallerTestClassWrapper.create},
            {},
            {},
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'request': 4,
              r'initializeMirror': 0,
              r'caller': 0,
              r'caller=': 4,
              r'create': 4
            }),
        r.NonGenericClassMirrorImpl(
            r'CallerWrapper',
            r'.CallerWrapper',
            134217735,
            2,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {},
            {},
            {
              r'': (bool b) =>
                  (caller) => b ? prefix3.CallerWrapper(caller) : null
            },
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'func0': 5,
              r'func1': 5,
              r'func2': 4,
              r'test0': 4,
              r'test1': 4,
              r'initializeMirror': 0,
              r'caller': 0
            }),
        r.NonGenericClassMirrorImpl(
            r'GqlConnection',
            r'.GqlConnection',
            134217735,
            3,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {r'create': () => prefix4.GqlConnection.create},
            {},
            {},
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'post': 4,
              r'initializeMirror': 0,
              r'connection': 0,
              r'connection=': 4,
              r'type': 0,
              r'name': 0,
              r'networkId': 0,
              r'group': 0,
              r'create': 6
            }),
        r.NonGenericClassMirrorImpl(
            r'JrpcConnection',
            r'.JrpcConnection',
            134217735,
            4,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {r'create': () => prefix5.JrpcConnection.create},
            {},
            {},
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'post': 4,
              r'initializeMirror': 0,
              r'connection': 0,
              r'connection=': 4,
              r'type': 0,
              r'name': 0,
              r'networkId': 0,
              r'group': 0,
              r'create': 6
            }),
        r.NonGenericClassMirrorImpl(
            r'LedgerConnection',
            r'.LedgerConnection',
            134217735,
            5,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {r'create': () => prefix6.LedgerConnection.create},
            {},
            {},
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'getPublicKey': 4,
              r'initializeMirror': 0,
              r'connection': 0,
              r'connection=': 4,
              r'create': 7
            }),
        r.NonGenericClassMirrorImpl(
            r'Storage',
            r'.Storage',
            134217735,
            6,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {r'create': () => prefix7.Storage.create},
            {},
            {},
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'get': 4,
              r'set': 9,
              r'setUnchecked': 9,
              r'remove': 9,
              r'removeUnchecked': 9,
              r'initializeMirror': 0,
              r'storage': 0,
              r'storage=': 4,
              r'create': 8
            }),
        r.NonGenericClassMirrorImpl(
            r'TokenWallet',
            r'.TokenWallet',
            134217735,
            7,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {
              r'subscribe': () => prefix8.TokenWallet.subscribe,
              r'getTokenWalletDetails': () =>
                  prefix8.TokenWallet.getTokenWalletDetails,
              r'getTokenRootDetailsFromTokenWallet': () =>
                  prefix8.TokenWallet.getTokenRootDetailsFromTokenWallet
            },
            {},
            {},
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'getContractState': 0,
              r'prepareTransfer': 11,
              r'refresh': 0,
              r'preloadTransactions': 2,
              r'handleBlock': 3,
              r'onBalanceChanged': 4,
              r'onTransactionsFound': 4,
              r'initializeMirror': 0,
              r'wallet': 0,
              r'wallet=': 4,
              r'transport': 0,
              r'balance': 0,
              r'balance=': 4,
              r'address': 0,
              r'address=': 4,
              r'owner': 0,
              r'owner=': 4,
              r'symbol': 0,
              r'symbol=': 4,
              r'version': 0,
              r'version=': 4,
              r'contractState': 0,
              r'fieldUpdatesStream': 0,
              r'onBalanceChangedStream': 0,
              r'onTransactionsFoundStream': 0,
              r'subscribe': 10,
              r'getTokenWalletDetails': 12,
              r'getTokenRootDetailsFromTokenWallet': 12
            }),
        r.NonGenericClassMirrorImpl(
            r'TonWallet',
            r'.TonWallet',
            134217735,
            8,
            const prefix0.Reflector(),
            const <int>[-1],
            null,
            null,
            -1,
            {
              r'subscribe': () => prefix9.TonWallet.subscribe,
              r'subscribeByAddress': () => prefix9.TonWallet.subscribeByAddress,
              r'subscribeByExistingWallet': () =>
                  prefix9.TonWallet.subscribeByExistingWallet,
              r'findExistingWallets': () =>
                  prefix9.TonWallet.findExistingWallets,
              r'getExistingWalletInfo': () =>
                  prefix9.TonWallet.getExistingWalletInfo,
              r'getWalletCustodians': () =>
                  prefix9.TonWallet.getWalletCustodians
            },
            {},
            {},
            -1,
            -1,
            const <int>[-1],
            null,
            {
              r'==': 4,
              r'toString': 0,
              r'noSuchMethod': 4,
              r'hashCode': 0,
              r'runtimeType': 0,
              r'init': 0,
              r'dispose': 0,
              r'instanceHash': 0,
              r'avoidCall': 0,
              r'getContractState': 0,
              r'getPendingTransactions': 0,
              r'getPollingMethod': 0,
              r'getUnconfirmedTransactions': 0,
              r'getCustodians': 0,
              r'prepareDeploy': 15,
              r'prepareDeployWithMultipleOwners': 16,
              r'prepareTransfer': 17,
              r'prepareConfirmTransaction': 18,
              r'estimateFees': 1,
              r'send': 1,
              r'refresh': 0,
              r'preloadTransactions': 2,
              r'handleBlock': 3,
              r'onMessageSent': 4,
              r'onMessageExpired': 4,
              r'onStateChanged': 4,
              r'onTransactionsFound': 4,
              r'initializeMirror': 0,
              r'wallet': 0,
              r'wallet=': 4,
              r'transport': 0,
              r'details': 0,
              r'details=': 4,
              r'publicKey': 0,
              r'publicKey=': 4,
              r'address': 0,
              r'address=': 4,
              r'walletType': 0,
              r'walletType=': 4,
              r'workchain': 0,
              r'workchain=': 4,
              r'custodians': 0,
              r'contractState': 0,
              r'pendingTransactions': 0,
              r'pollingMethod': 0,
              r'unconfirmedTransactions': 0,
              r'fieldUpdatesStream': 0,
              r'onMessageSentStream': 0,
              r'onMessageExpiredStream': 0,
              r'onStateChangedStream': 0,
              r'onTransactionsFoundStream': 0,
              r'subscribe': 13,
              r'subscribeByAddress': 0,
              r'subscribeByExistingWallet': 14,
              r'findExistingWallets': 19,
              r'getExistingWalletInfo': 0,
              r'getWalletCustodians': 0
            })
      ],
      null,
      null,
      <Type>[
        prefix1.GenericContract,
        prefix2.CallerTestClassWrapper,
        prefix3.CallerWrapper,
        prefix4.GqlConnection,
        prefix5.JrpcConnection,
        prefix6.LedgerConnection,
        prefix7.Storage,
        prefix8.TokenWallet,
        prefix9.TonWallet
      ],
      9,
      {
        r'==': (dynamic instance) => (x) => instance == x,
        r'toString': (dynamic instance) => instance.toString,
        r'noSuchMethod': (dynamic instance) => instance.noSuchMethod,
        r'hashCode': (dynamic instance) => instance.hashCode,
        r'runtimeType': (dynamic instance) => instance.runtimeType,
        r'init': (dynamic instance) => instance.init,
        r'dispose': (dynamic instance) => instance.dispose,
        r'instanceHash': (dynamic instance) => instance.instanceHash,
        r'avoidCall': (dynamic instance) => instance.avoidCall,
        r'getContractState': (dynamic instance) => instance.getContractState,
        r'getPendingTransactions': (dynamic instance) =>
            instance.getPendingTransactions,
        r'getPollingMethod': (dynamic instance) => instance.getPollingMethod,
        r'executeTransactionLocally': (dynamic instance) =>
            instance.executeTransactionLocally,
        r'estimateFees': (dynamic instance) => instance.estimateFees,
        r'send': (dynamic instance) => instance.send,
        r'refresh': (dynamic instance) => instance.refresh,
        r'preloadTransactions': (dynamic instance) =>
            instance.preloadTransactions,
        r'handleBlock': (dynamic instance) => instance.handleBlock,
        r'onMessageSent': (dynamic instance) => instance.onMessageSent,
        r'onMessageExpired': (dynamic instance) => instance.onMessageExpired,
        r'onStateChanged': (dynamic instance) => instance.onStateChanged,
        r'onTransactionsFound': (dynamic instance) =>
            instance.onTransactionsFound,
        r'initializeMirror': (dynamic instance) => instance.initializeMirror,
        r'contract': (dynamic instance) => instance.contract,
        r'transport': (dynamic instance) => instance.transport,
        r'address': (dynamic instance) => instance.address,
        r'contractState': (dynamic instance) => instance.contractState,
        r'pendingTransactions': (dynamic instance) =>
            instance.pendingTransactions,
        r'pollingMethod': (dynamic instance) => instance.pollingMethod,
        r'fieldUpdatesStream': (dynamic instance) =>
            instance.fieldUpdatesStream,
        r'onMessageSentStream': (dynamic instance) =>
            instance.onMessageSentStream,
        r'onMessageExpiredStream': (dynamic instance) =>
            instance.onMessageExpiredStream,
        r'onStateChangedStream': (dynamic instance) =>
            instance.onStateChangedStream,
        r'onTransactionsFoundStream': (dynamic instance) =>
            instance.onTransactionsFoundStream,
        r'request': (dynamic instance) => instance.request,
        r'caller': (dynamic instance) => instance.caller,
        r'func0': (dynamic instance) => instance.func0,
        r'func1': (dynamic instance) => instance.func1,
        r'func2': (dynamic instance) => instance.func2,
        r'test0': (dynamic instance) => instance.test0,
        r'test1': (dynamic instance) => instance.test1,
        r'post': (dynamic instance) => instance.post,
        r'connection': (dynamic instance) => instance.connection,
        r'type': (dynamic instance) => instance.type,
        r'name': (dynamic instance) => instance.name,
        r'networkId': (dynamic instance) => instance.networkId,
        r'group': (dynamic instance) => instance.group,
        r'getPublicKey': (dynamic instance) => instance.getPublicKey,
        r'get': (dynamic instance) => instance.get,
        r'set': (dynamic instance) => instance.set,
        r'setUnchecked': (dynamic instance) => instance.setUnchecked,
        r'remove': (dynamic instance) => instance.remove,
        r'removeUnchecked': (dynamic instance) => instance.removeUnchecked,
        r'storage': (dynamic instance) => instance.storage,
        r'prepareTransfer': (dynamic instance) => instance.prepareTransfer,
        r'onBalanceChanged': (dynamic instance) => instance.onBalanceChanged,
        r'wallet': (dynamic instance) => instance.wallet,
        r'balance': (dynamic instance) => instance.balance,
        r'owner': (dynamic instance) => instance.owner,
        r'symbol': (dynamic instance) => instance.symbol,
        r'version': (dynamic instance) => instance.version,
        r'onBalanceChangedStream': (dynamic instance) =>
            instance.onBalanceChangedStream,
        r'getUnconfirmedTransactions': (dynamic instance) =>
            instance.getUnconfirmedTransactions,
        r'getCustodians': (dynamic instance) => instance.getCustodians,
        r'prepareDeploy': (dynamic instance) => instance.prepareDeploy,
        r'prepareDeployWithMultipleOwners': (dynamic instance) =>
            instance.prepareDeployWithMultipleOwners,
        r'prepareConfirmTransaction': (dynamic instance) =>
            instance.prepareConfirmTransaction,
        r'details': (dynamic instance) => instance.details,
        r'publicKey': (dynamic instance) => instance.publicKey,
        r'walletType': (dynamic instance) => instance.walletType,
        r'workchain': (dynamic instance) => instance.workchain,
        r'custodians': (dynamic instance) => instance.custodians,
        r'unconfirmedTransactions': (dynamic instance) =>
            instance.unconfirmedTransactions
      },
      {
        r'contract=': (dynamic instance, value) => instance.contract = value,
        r'address=': (dynamic instance, value) => instance.address = value,
        r'caller=': (dynamic instance, value) => instance.caller = value,
        r'connection=': (dynamic instance, value) =>
            instance.connection = value,
        r'storage=': (dynamic instance, value) => instance.storage = value,
        r'wallet=': (dynamic instance, value) => instance.wallet = value,
        r'balance=': (dynamic instance, value) => instance.balance = value,
        r'owner=': (dynamic instance, value) => instance.owner = value,
        r'symbol=': (dynamic instance, value) => instance.symbol = value,
        r'version=': (dynamic instance, value) => instance.version = value,
        r'details=': (dynamic instance, value) => instance.details = value,
        r'publicKey=': (dynamic instance, value) => instance.publicKey = value,
        r'walletType=': (dynamic instance, value) =>
            instance.walletType = value,
        r'workchain=': (dynamic instance, value) => instance.workchain = value
      },
      null,
      [
        const [
          0,
          0,
          const [#transport, #address, #preloadTransactions]
        ],
        const [
          0,
          0,
          const [#signedMessage, #options]
        ],
        const [
          0,
          0,
          const [#fromLt]
        ],
        const [
          0,
          0,
          const [#block]
        ],
        const [1, 0, null],
        const [
          3,
          0,
          const [#arg0, #arg1]
        ],
        const [
          0,
          0,
          const [#post, #get, #settings, #name, #group, #networkId]
        ],
        const [
          0,
          0,
          const [#getPublicKey, #connectionSign]
        ],
        const [
          0,
          0,
          const [#get, #set, #setUnchecked, #remove, #removeUnchecked]
        ],
        const [
          0,
          0,
          const [#key, #value]
        ],
        const [
          0,
          0,
          const [#transport, #owner, #rootTokenContract]
        ],
        const [
          0,
          0,
          const [
            #destination,
            #amount,
            #notifyReceiver,
            #attachedAmount,
            #payload
          ]
        ],
        const [
          0,
          0,
          const [#transport, #tokenWalletAddress]
        ],
        const [
          0,
          0,
          const [#transport, #workchainId, #publicKey, #walletType]
        ],
        const [
          0,
          0,
          const [#transport, #existingWallet]
        ],
        const [
          0,
          0,
          const [#expiration]
        ],
        const [
          0,
          0,
          const [#expiration, #custodians, #reqConfirms]
        ],
        const [
          0,
          0,
          const [
            #contractState,
            #publicKey,
            #destination,
            #amount,
            #bounce,
            #body,
            #expiration
          ]
        ],
        const [
          0,
          0,
          const [#contractState, #publicKey, #transactionId, #expiration]
        ],
        const [
          0,
          0,
          const [#transport, #workchainId, #publicKey, #walletTypes]
        ]
      ])
};

final _memberSymbolMap = null;

void initializeReflectable() {
  r.data = _data;
  r.memberSymbolMap = _memberSymbolMap;
}
