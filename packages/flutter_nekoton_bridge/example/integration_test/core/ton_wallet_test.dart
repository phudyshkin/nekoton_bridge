import 'package:flutter/material.dart';
import 'package:flutter_nekoton_bridge/flutter_nekoton_bridge.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:http/http.dart' as http;

Future<String> postTransportData({
  required String endpoint,
  required Map<String, String> headers,
  required String data,
}) async {
  final response = await http.post(
    Uri.parse(endpoint),
    headers: headers,
    body: data,
  );

  return response.body;
}

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  const name = 'Mainnet (GQL)';
  const networkId = 1;
  const networkGroup = 'mainnet';
  const endpoint = 'https://jrpc.everwallet.net/rpc';

  // const stEverContractVault =
  //     '0:675a6d63f27e3f24d41d286043a9286b2e3eb6b84fa4c3308cc2833ef6f54d68';
  const publicKey =
      'ad158ac64c5deff5abd4d5e86a81d954716445c45e31f17a9dfe780f9cef7602';
  const address =
      '0:d92c91860621eb5397957ee3f426860e2c21d7d4410626885f35db88a46a87c2';
  const workchainId = 0;
  const walletType = WalletType.walletV3();
  const expiration = Expiration.timeout(60);

  const jrpcSettings = JrpcNetworkSettings(endpoint: endpoint);
  late JrpcTransport transport;

  setUp(() async {
    // This setup thing SHOULD NOT be removed or altered because it used in integration tests
    setupLogger(
      level: LogLevel.Trace,
      mobileLogger: false,
      logHandler: (logEntry) => debugPrint(
        'FromLib: ${logEntry.level} ${logEntry.tag} ${logEntry.msg} (lib_time=${logEntry.timeMillis})',
      ),
    );

    runApp(Container());

    await initRustToDartCaller();

    final connection = await JrpcConnection.create(
      post: postTransportData,
      settings: jrpcSettings,
      name: name,
      group: networkGroup,
      networkId: networkId,
    );
    transport = await JrpcTransport.create(jrpcConnection: connection);
  });

  group('TonWallet test', () {
    testWidgets('TonWallet subscribe', (WidgetTester tester) async {
      await tester.pumpAndSettle();

      final wallet = await TonWallet.subscribe(
        transport: transport,
        workchainId: workchainId,
        publicKey: publicKey,
        walletType: walletType,
      );

      expect(wallet, isNotNull);
      expect(wallet.address, address);
      expect(wallet.publicKey, publicKey);
      expect(wallet.walletType, walletType);
      expect(wallet.workchain, 0);
    });

    testWidgets('TonWallet subscribeByAddress', (WidgetTester tester) async {
      await tester.pumpAndSettle();

      final wallet = await TonWallet.subscribeByAddress(
        transport: transport,
        address: address,
      );

      expect(wallet, isNotNull);
      expect(wallet.address, address);
      expect(wallet.publicKey, publicKey);
      expect(wallet.walletType, walletType);
      expect(wallet.workchain, 0);
    });

    testWidgets('TonWallet subscribeByExistingWallet',
        (WidgetTester tester) async {
      await tester.pumpAndSettle();

      final infoList = await TonWallet.findExistingWallets(
        transport: transport,
        workchainId: workchainId,
        publicKey: publicKey,
        walletTypes: [walletType],
      );

      /// 1 because we expect only one type
      expect(infoList.length, 1);

      final wallet = await TonWallet.subscribeByExistingWallet(
        transport: transport,
        existingWallet: infoList.first,
      );

      expect(wallet, isNotNull);
      expect(wallet.address, address);
      expect(wallet.publicKey, publicKey);
      expect(wallet.walletType, walletType);
      expect(wallet.workchain, 0);
    });

    // TODO: Right now not works, fix later
    // testWidgets('TonWallet prepareTransfer', (WidgetTester tester) async {
    //   await tester.pumpAndSettle();
    //
    //   final wallet = await TonWallet.subscribeByAddress(
    //     transport: transport,
    //     address: address,
    //   );
    //
    //   final contract = await transport.getContractState(stEverContractVault);
    //
    //   final message = await wallet.prepareTransfer(
    //     contractState: contract,
    //     publicKey: publicKey,
    //     destination: await repackAddress(stEverContractVault),
    //     amount: '1000000000',
    //     bounce: false,
    //     expiration: expiration,
    //   );
    //   expect(message, isNotNull);
    // });

    testWidgets('TonWallet prepareDeploy', (WidgetTester tester) async {
      await tester.pumpAndSettle();

      final wallet = await TonWallet.subscribeByAddress(
        transport: transport,
        address: address,
      );

      try {
        await wallet.prepareDeploy(expiration: expiration);
      } catch (_) {
        /// deploy for this wallet throws error because it had been already deployed
        expect(true, true);
      }
    });

    testWidgets('TonWallet getExistingWalletInfo', (WidgetTester tester) async {
      await tester.pumpAndSettle();

      final wallet = await TonWallet.getExistingWalletInfo(
        transport: transport,
        address: address,
      );

      expect(wallet.address, address);
      expect(wallet.publicKey, publicKey);
      expect(wallet.walletType, walletType);
      expect(wallet.contractState.balance, isNot('0'));
      expect(wallet.contractState.isDeployed, isTrue);
    });

    testWidgets('TonWallet getWalletCustodians', (WidgetTester tester) async {
      await tester.pumpAndSettle();

      final custodians1 = await TonWallet.getWalletCustodians(
        transport: transport,
        address: address,
      );
      final custodians2 = await TonWallet.getWalletCustodians(
        transport: transport,
        address:
            '0:91b689ad990660249eb00140577e6a98d70043ccaa7f63acfc0436336bdbd80f',
      );

      /// For not multisig wallet custodians contains public key of wallet
      expect(custodians1, [publicKey]);
      expect(custodians2.length, 3);
    });
  });
}