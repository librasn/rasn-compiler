use rasn_compiler_tests::helpers::decode_hex;
use rasn_compiler_tests::asn1::rasn_generated::*;

#[test]
fn decodes_its_pdu_header() {
    let binary_cam = decode_hex("0202de140ce5c7c0405ab23d82ce2781e9a278274bc633fa54587ca0a27e8302968a9733ff82001a103fe0143980106e0075801158ce0002f03adc08c4c800015781d620469633800abc0edb0239319c0055e075081185900002af03a0c0912c800016781c9e0565640000c3c0e0902dbb19c006de058d810218ce0035f0155c0006c67000df808d5fde662700073c0476fd67319c0058604137d31589c006dc").unwrap();

    let result: ItsPduHeader = rasn::uper::decode(&binary_cam).unwrap();
    assert_eq!(result.message_i_d, 2);
    assert_eq!(result.protocol_version, 2);
    assert_eq!(result.station_i_d.0, 3725855973);
}

#[test]
fn decodes_cam_1() {
    let binary_cam: Vec<u8> = decode_hex("0202de140ce5c7c0405ab23d82ce2781e9a278274bc633fa54587ca0a27e8302968a9733ff82001a103fe0143980106e0075801158ce0002f03adc08c4c800015781d620469633800abc0edb0239319c0055e075081185900002af03a0c0912c800016781c9e0565640000c3c0e0902dbb19c006de058d810218ce0035f0155c0006c67000df808d5fde662700073c0476fd67319c0058604137d31589c006dc").unwrap();

    let result: CAM = rasn::uper::decode(&binary_cam).unwrap();
    if let LowFrequencyContainer::BasicVehicleContainerLowFrequency(lfct) =
        result.cam.cam_parameters.low_frequency_container.unwrap()
    {
        assert_eq!(
            440,
            *lfct.path_history
                .0
                .last()
                .unwrap()
                .path_delta_time
                .as_ref()
                .unwrap()
                .0
                .to_u32_digits()
                .1
                .first()
                .unwrap()
        );
    } else {
        panic!("Not the right LowFrequencyContainer");
    }
 }

#[test]
fn decodes_cam_2() {
    let binary_cam: Vec<u8> = decode_hex("02024810c86ee3d800fab1dcc3ae27aef4c204204000345bd0a47014d712b61c1a1003df20003e8894d713b01c1a0fead2a0a6b8a550e0d07e8e").unwrap();
    let result: CAM = rasn::uper::decode(&binary_cam).unwrap();
    if let HighFrequencyContainer::RsuContainerHighFrequency(rsuhf) =
        result.cam.cam_parameters.high_frequency_container
    {
        assert_eq!(
            85880135,
            rsuhf
                .protected_communication_zones_r_s_u
                .unwrap()
                .0
                .last()
                .unwrap()
                .protected_zone_longitude
                .0
        );
    } else {
        panic!("Not the right LowFrequencyContainer");
    }
}

#[test]
fn decodes_other_test_cams() {
    let cams: Vec<Vec<u8>> = vec![decode_hex("EE0959C86154DF864998CF44058677306E3AFB7CD5DB3477A6855C00").unwrap(),
    decode_hex("47FD7DC67AD4EC262E5545C4218FE5253C8EDDE55B675E541A8900").unwrap(),
    decode_hex("DC1A7FFFFFFF1B470846672DDAED9D8DFF29BEE7844C17B0E0418AD22EBA9A0C481CF2510ACBAF2E09A6911C9465B659F85EB746C0").unwrap(),
    decode_hex("94137FFFFFFF7ECE290A40FC2B08ED4C4DB3568617E432D52A63CE151638CD9E79BC54DE53267745CD774D9A741E3A5B5C91732526A17C").unwrap(),
    decode_hex("3DC57FFFFFFFD73D0E6AEC9FA06F946B00C04EC7001E6BFF8080").unwrap()];

    let svc_present = vec![false, true, false, true, false];

    for i in 0..5 {
        let result: CAM = rasn::uper::decode(&cams[i]).unwrap();
        assert_eq!(
            svc_present[i],
            result.cam.cam_parameters.special_vehicle_container.is_some()
        );
    }
}

#[test]
fn decodes_cpm() {
    let binary_cpm: Vec<u8> = decode_hex("010ea6c052477a3f700b56402701c4d78162b9eb9c0006ced283a63ed7d00bfeca4cca0ccfffd014c44007b21003e819019032008a03bb83e92f94fd90657ffff5ffffd448fce00ee5004503ddc1f534ca7f6cb2bffffaffffe9c1fe7007728022826ee0fb55653fb4995ffffd7ffff607441003b940114157707d59329fe7fcaffffebffffb42468801dca008a12bb83e25794fe03e57ffff5ffffd2d4fce00ee500450e5dc1f043ca808af2bffffaffffed461e200772802088eee0f9d5653f38b95ffffd7ffff5c23f001b940115a37707bf5f2a0115caffffebffffa35df9c01dca008a35bb83ebc1950088657ffff5ffffd51dfce00ee50045645dc1ef1bca7e8a72bffffaffffed461e2007728022afaee0f822e53feb395ffffd7ffff4c9bf3803b940114337707bbf329f7a5caffffebffffac1ff9c00dca008a42bb83dd5594fc49e57ffff5ffffd4e9fce00ee5004502ddc1f4f7ca7ef6f2bffffaffffea1efe70037280228feee0f86e653f98795ffffd7ffff625861003b9401144d7707c37b29fef1caffffebffffb4586c801dca008a2fbb83e08b950042657ffff5ffffd2abfce00ee5004520ddc1efceca7eb672bf3dfafc2fed0619200372802093aee0f993e53f2d995ffffd7ffff5c23f001b9401049f7707bcf329f722caffafebf31fb6829000dca008a4cbb83e0ff94fc5ae57ffff5ffffd327fce00ee5004129ddc1ef21ca7fab72bffffaffffeb847e0037280208eeee0fa17e53f4b3960477d805ff43c7f001b940104a57707d5172a0167cafffbec03cfa739f800dca008255bb83de5194ff94657ffff5ffffd601fc006e5004518ddc1f16bca7f0af2c1a2faf613ee50b12003728d0").unwrap();

    let result: CPM = rasn::uper::decode(&binary_cpm).unwrap();
    assert_eq!(26, result.cpm.cpm_parameters.number_of_perceived_objects.0);
}

#[test]
fn decodes_denm() {
    let binary_denm: Vec<u8> = decode_hex("0201e0fd1d37e7465aa8bc8006910d64c90404435932459d59019207135f2142be2be000186a09880050144018030014fb843f0a2fdd6bfb00c50a06022d297ff59fffe606805f9c00ff000d3124049be0064802ad89201387001bc0394c328112f801a5fed56144095fc00d2ffc93110015de00577f46580602c86ffd1bfccec7d80bb77fd59fcdb70e4289fbff7b008f33cc0577e00418010d86f0176f0019bfeeeca803a337fc66041e656810fdc002f0021317406d5e004a7ff1d8bf015170037bf862cdf013d78007a002164a001fdbffd2fff8b2aa0270dffb9800b58e700f9effb33fffec4b813d77ff2dffb362e80af9c000900c1b1240784dff12808119f003816ff7f401ccc828423f7ffadfffe602c0531c0278ff0ab1100671dffce7ff999fa01daefff440172cc880c7f8001a00ee63d80d0abffe0ffdcaec2093d5ff4b7fffd8970d1ab00533fd64c9b80c1d80145fe3b64640545c023cff922fee0883e0081801458e2013870052c06acb72015df800a1fff763880671c01d7015d31920577dfff58060d7c00106420480600c").unwrap();

    let result: DENM = rasn::uper::decode(&binary_denm).unwrap();
    assert_eq!(
        true,
        result.denm.alacarte.unwrap().stationary_vehicle.is_none()
    );
}

#[test]
fn decodes_demo_denm() {
    let binary_denm: Vec<u8> = decode_hex(
        "0201000B3C198000059E0C80008000000000000000000005569D840713070C0FFFFFFE11DBBA1F051C0200",
    )
    .unwrap();

    let result: DENM = rasn::uper::decode(&binary_denm).unwrap();
    assert_eq!(1, result.denm.situation.unwrap().event_type.cause_code.0)
}

#[test]
fn decodes_other_test_denms() {
    let denms: Vec<Vec<u8>> = vec![decode_hex("AD5857AA21920D3FFFFFFFEF38800FFFFFFFE003FFFFFFFD35FFFA1C405FFA4F94AC61DD271DA8613300D8").unwrap(),
    decode_hex("40CF7A0C004387BFFFFFFFC439800FFFFFFFE003FFFFFFFC2E9399847430CCAA56BC82CBCAE9D10B91DA489753CE92A0CA3AD082C3854477A3").unwrap(),
    decode_hex("34447FFFFFFFC93FFFFFFFB97A000FFFFFFFE003FFFFFFFB4793191190EEF338B9D9F6DEA590B9146BB84B56C10257C828848DD11831644008").unwrap(),
    decode_hex("A66E7FFFFFFF01BFFFFFFFAAF8000FFFFFFFE003FFFFFFF85FC0FF8780392A1F7754B7EB5E2C871B91317A22").unwrap(),
    decode_hex("520E5225F74FC33FFFFFFFE10E800FFFFFFFE003FFFFFFF93A2CFFD700AB3E74491121659AFA31D8D83A88BAD31F108FFC2877CD6E82BCFC85B7D29DB54E7F3F1593952C031B6B4000").unwrap()
    ];

    for i in 0..5 {
        let result: DENM = rasn::uper::decode(&denms[i]).unwrap();
        assert_eq!(true, result.denm.alacarte.is_none())
    }
}

#[test]
fn decodes_mapem_1() {
    let binary_mapem: Vec<u8> = decode_hex("020500003016080003094d8342fc9a94efb26b7193560c6e325cca0006138a08558f2236713c8327028a24482a480420000036d02fc0de2200b061c10fc05be4f4c3c7abc9878347f002c5a0000404405908084000002ceca22680460c05546623dd7dc2401242200000db40d500e280020ece061e011f102dbb76e1e6bdf820b068000102100642220000049dd5c0a611c6f4824e5f892011210000006da0787f1640010e407e83d47bd985efef062cd080c0582c0000818802a1100000024ef41efb09f57ca1034f5c9024d0800000367a06adaa00088f8e074646f7b108699bc487393f520b0b8000704100b6220000004b4ceae9781caeec43e35d292051a11000006ce08945d400109447b622467c818d51cf83137c02c16190000e0a2018c44400000969941cdd83eb1c407133cd241734021000002b38294ddd10055109c47da33483d8c2031ac0430b8810e1be70cd8501e058c4000383080c310108000005b4c62e50c08a887803900ec1770480ea84000002b68101e59d00040fede609af4a052f8d02eeabc1e8ce23e213b705616800080eb0c4000408040f604080000033f1f89610004102cc80a1002090410604080000033eec78dd00040fdb380a0f030a0411604080000033e6f59fd0004102cc80a12080b0412604080000033e3d499900040fdb380a11090c0413604080000033ee85fb100040ce3f40a14050d0414604080000036884de075000413240c0a13060e0").unwrap();

    let result: MAPEM = rasn::uper::decode(&binary_mapem).unwrap();
    assert_eq!(
        14,
        result
            .map
            .intersections
            .unwrap()
            .0
            .last()
            .unwrap()
            .lane_set
            .0
            .last()
            .unwrap()
            .connects_to
            .as_ref()
            .unwrap()
            .0
            .last()
            .unwrap()
            .connection_i_d
            .as_ref()
            .unwrap()
            .0
    );
}

#[test]
fn decodes_mapem_2() {
    let binary_mapem: Vec<u8> = decode_hex("0205e6e167b408000005566022a089a21b9bdf61f028200a00022000000657a94805e2bc69403895e21e0628af2f705dc55f429438148290000a4190000810090001100000002bfdabd7915ab517c24520d000069044000404034000440000000aff6b060258f2a983f148210002a4160001800110001100000012bd66c07095e4c202f0af0c502c8000a80008800000015f285f8acaf448edd6000c80008800000016101e02a4b0b9314380").unwrap();

    let result: MAPEM = rasn::uper::decode(&binary_mapem).unwrap();
    if let NodeListXY::Nodes(nodes) = result
        .map
        .intersections
        .unwrap()
        .0
        .last()
        .unwrap()
        .lane_set
        .0
        .last()
        .unwrap()
        .node_list
        .clone()
    {
        if let NodeOffsetPointXY::NodeXY6(node) = nodes.0.last().unwrap().delta.clone() {
            assert_eq!(2588, node.y.0);
        } else {
            panic!("Unexpected node type!")
        }
    } else {
        panic!("Unexpected node list type!")
    }
}

#[test]
fn decodes_other_test_mapems() {
    let mapems: Vec<Vec<u8>> = vec![
        decode_hex("020500003003080003094d8342fc9a94efb065719346d719b85cda000601ec18558fbbb8713d8403028a424817084400000b693c9ed41000407184435a81f0c161b0001402201b244400000169b79fb0034e3449207521100000ada4067e3e400109f898c235f9c709e5bc61783ffc334f8706966fb1586a0005812c450002c1090421088000016d310bd4ba000810014c6af63c902c46000280c40464888000012d4163f4d82bee6c12cd62481e684000000b6b5e223ad0004086076250c224058990000828809d910000004db5db8ba8045441dd2328e813704abd162b4208a8920d620108000011b5b593d6c8802a88787e0e17baefb22d5aac12320e8f6a08b4eae4004058510001830805210108000004b4c393eae141b87008160c4833884000002b6b82e775d1005510eddbb22c33b7e4f9de040957a2bfa7c11830e317ee8afe9e400405841000183880421100000005a54d9e970924e7c4802884400001368d0e6351000469f4c0089bc73e489ed60f02b02200040858e9000204880131100000005a0e0913d09050be203ac40420000002d07d495d008b1f89024d080000006d63ac4d4200088fb80ec160a8000414200ae4400000016b28a453c0b55904825684000003b6d99a33a110055107748cab70539812af5e51590008a88f754e121e8edfb03d1ec42c7b8fa0c938b0c6d8160b00012164808a8400000036a686514100043918ed02c1e0001430401d0880000002d1a64e1905027e32418c42000000db540d2fc08802a88b25b0917dcadafa20255eac75bb504544b551d42420b02800060d0414604080000036ce522dc100040eab4c0a150c0e0415604080000036d3ce39650004115cb40a140c0f0416604080000036b29e31e900040d5c680a1707100417604080000036bdfa2b05000412ab980a160711041b60408000003685e24ee900040db4860a1c0812041c6040800000368c364941000412537a0a1b081304224040800000368c325b7100040d43a00a230e1400232040800000369b9a63cd000412c460084880810000006d37e4c79200082598b4144a1e2a004a40810000006d176cb72a00081a774c10990102000000dab709e35400103b92e8289c4058009c8102000000dac119c9a40010448d1821420204000001b587b392a800208c1a90514868b801490204000001b56b53c6280020742570").unwrap(),
        decode_hex("020500003016080003094d8342fc9a94efb26b7193560c6e325cca0006138a08558f2236713c8327028a24482a480420000036d02fc0de2200b061c10fc05be4f4c3c7abc9878347f002c5a0000404405908084000002ceca22680460c05546623dd7dc2401242200000db40d500e280020ece061e011f102dbb76e1e6bdf820b068000102100642220000049dd5c0a611c6f4824e5f892011210000006da0787f1640010e407e83d47bd985efef062cd080c0582c0000818802a1100000024ef41efb09f57ca1034f5c9024d0800000367a06adaa00088f8e074646f7b108699bc487393f520b0b8000704100b6220000004b4ceae9781caeec43e35d292051a11000006ce08945d400109447b622467c818d51cf83137c02c16190000e0a2018c44400000969941cdd83eb1c407133cd241734021000002b38294ddd10055109c47da33483d8c2031ac0430b8810e1be70cd8501e058c4000383080c310108000005b4c62e50c08a887803900ec1770480ea84000002b68101e59d00040fede609af4a052f8d02eeabc1e8ce23e213b705616800080eb0c4000408040f604080000033f1f89610004102cc80a1002090410604080000033eec78dd00040fdb380a0f030a0411604080000033e6f59fd0004102cc80a12080b0412604080000033e3d499900040fdb380a11090c0413604080000033ee85fb100040ce3f40a14050d0414604080000036884de075000413240c0a13060e0").unwrap(),
        decode_hex("576D7DE039151870AE2541F0CAD9FE988369E852745C33E877A4FBDC80C11498500000C103C7EEE000C103D43B60040EA72CE30016384BF187A58ECDD9D2F3BB7EC5FEB81340C092303D214CB2B04F81BE5CA8030205001940C081401D37C44C39F6B5DD639539B469D6F819AFE5FC487DA451DB887469F0AF5D16924E9665A21109804A328220D5B3B76CC443975B0181B702580036069F5FB470086A014B0326A06040A00D406040A007CC5E0F13911234B31488178B013AE8185002E92C94DC015327C48018D83158DA9499A1A1658980ED5AC0B361860E0E06040A001D1BCA4142DCF011CCFBAC5ABE8AC808463BC37906201890038D0181028026304CD3625C682E763D50E108D5350DBF7C54C62156DBD8FD51BC03020500FE0302050000D3813CA40C081401E98946510414912A0DAE7D52CD5E42530A44300D7ADDCD0662C04C26C1A4C7018094778D4F91C58CA0C111B6818618202DAFDE1302A94B1C620E0185380002A4B5202BE38410428F2030205005068BCDA46DCE25F262A0D26AA0D717E6F4CC342853E94086648093D116C3499A540C08CD4126C79E33A7E006A58B870A7F40C474561956241FE8FBF2100337001CF8529D4DFD857E9901FC41AD11492504C62C00C081401A33030205008C0302050002030205000521F0310606040A00E480602003A0609A330B3D313A4DC3B2B66B35B328AA9CEC3800D44648D3C3D2C5280ECA06040A00").unwrap(),
        decode_hex("3BB87FFFFFFF441DAE0960A81E9E7BDE7813934180449FA86F7AACC2449FD7D3439D480C7703B2F405C030360ADB0E2C02745EF48D86645B6DB55631954106640189191020671DB5A4025ADEF250274861C821F02033271F42C20464E05002433203010003207042110000000000003684EA8AA0598A3B85C2C604E908DAFAAAC04F22A04540030850502E38BB489230411AE59F12E751514841E9458463509C657E0849E108184320302B8F1C743A6018A997027529DA572FAF602F9A8E137502C92C7175110039924A27CC031900C0D1DDA27640294038046094206CC609442E5CA48040FEE2455437C746D3F7CDEAD064EA2E2137F85AD99B0181B592280000215AD6FE9CF69380B50D390182890138B9718180800180800E2E45A0F100EEC0C74034360EE040300C41AC84AC2A4C273B1B043CB03006040A008E4F71AC00C2C00A8029FCB3F36B37114000C2400FE5676606A55F5880C3000C70CF133C10181028039800305382BE23A0002C33A26A9A1AF002C00D017A003F20AFF4C62E90CD1921121680EA441A05FF3A87A22C09BC81007F3E1086629036C0B8C90A0082DCF625944003D7499F990010AD58BBFF504910D66CE0").unwrap(),
        decode_hex("AFF94C85BEAF2D4960303E7E7934ECCDAF3481885341917A3EF26F4931E098018212A8D38070ED00606B3EAB8ABA2BC9D7C7861D5BE4419BA05E283058202DB52CFC0A658651886414F04C35030820604DDE2B5072A0030920E05711E3C6BC1E8016DD614038EEB82A57BE888CDA54D362A59E717846A197BDAE10A965975C42E2806410EC6C0F4CF40DCEA100304213DFB0015E27D297101691F068C2CADB1AB3F81812320564AEDB36B29680B075BBDF00630054C41A9CF108142D20DB147022007A018E31BC2E531E4D0F896244ECD58416D89025B8401811F24159009601987F742C1A0A99F8C6742D50DFF88855AB644669AE236066981A639C21689F397E0096BC0037CA63EB03F06401808024E03F93A4CEE84CA108F6FF1D6EEEB0B3748119A3A818700160832F18F467CFBE9CF261668019F005CC3B594740104A1405CCB60D768056E1AB206C28C0826500C88D8015889B016EFFE01C1013C5866B8147AD2318DA663043FFFFFFFFFFFC64CC4CD02F421006040A00B206040A0002B621940C09A8800C09A5013C140C04000C04C0").unwrap(),
        decode_hex("87635BE9934749B4239E508CEE2368591B08C41F69A7CC0060842B858000C5C2003F1F848254846D452299E0181B226C86758017401167B09D33DA1E150749619471E1755201A80C08140171F3F3C9A7666D7982A06485F2E9E3AA724A7E60583C400C56ACC93C092551F16405CE58CB3C600C4001D900C081400D506C96F4BDC194AA30099030205003940C08140009CC46A018080569201809502503014003013E970403014600C04000").unwrap(),
        decode_hex("8CF37FFFFFFF345635A129B08CFB437CD852B75677B0243201820C1F7CC063B0021C1D5FFA9316B8181BD40E0480A76F3BED1D1CA4D9485B030205001218B25E35AC65C26D89E5936FE1A0F39110CFA60D2E779015E64DE5E12C5094181F729242000654E50C096979CDC530022000000000003FD2CC0A4EF093C3DD8080ACF06E940B0A31B84B69808FD87FBB80FE903020500130311F40C05100C198420134063F15EE3CD806DA806226500A728448C084CDED6B341009BBB16000B707FF8A2177D1DB9F2819080435640A50291C111499D7263CD1BE119005035A0D4E04182C058181028034022018B0009D5C44E887E5C80417872206C4D83258DB76AABD826720128428D36F6AFEB7CE729CF41D78423827818102800A8F80A3EE7DCF0C7D81EC0AAC0C081403E80C081400B80C0814000AD78FACD689DE7CE773F5D7C4D4060830191C0006083EB2DC000608C9F0E20008E042159CED9CE32EBEAE6060476B0A22A84703F3723E008C91807A0119688764D91E40F5B8EB90F912C5E65DA99252A18A4030161A9BF0683517CC859008074729285800BB500ACF0206040A0001ACA014571A010EA5F3A4E251E7413EA0EF80").unwrap()
    ];

    for i in 0..mapems.len() {
        let _result: MAPEM = rasn::uper::decode(&mapems[i]).unwrap();
    }
}

#[test]
fn decodes_spatem() {
    let binary_spatem: Vec<u8> = decode_hex("02040000301600384a6c1a17e4d4a77d935b8c9ab0637192e65000309c5040000ea71d4ff01000228ce2f944ff635b634871adb1aea1aea78010a1b8be2940a0d6d8d15c6b6c6c0c6c0de006280e2dfa68f635b634571adb1b031b0378020a1b8d8b8dc00da70d11c6d386d4c6d4de00a28ae35f236c4366034371b301bb71bad18030a038af798650d6d8d15c6b6c6c0c6c0c600e28ce35fc36ce366a34871b351b441b4478040a1b8d8b8dc00da70d15c6d386e3c6e146012280e2dfa5cd035b634571adb1b1c1b0d180").unwrap();

    let result: SPATEM = rasn::uper::decode(&binary_spatem).unwrap();
    assert_eq!(
        3,
        result
            .spat
            .intersections
            .0
            .last()
            .unwrap()
            .states
            .0
            .last()
            .unwrap()
            .state_time_speed
            .0
            .last()
            .unwrap()
            .timing
            .as_ref()
            .unwrap()
            .confidence
            .as_ref()
            .unwrap()
            .0
    );
}

#[test]
fn decodes_other_spatems() {
    let spatems: Vec<Vec<u8>> = vec![
    decode_hex("020400122a80413376018800184e20102001337618a10320134373ae33afc3aed5a339d971e141e0ca50dcee20f230f1e128ce77a67b707b4891000400220334673ad43be23b01021b9d9c1e259db51119cee0cf5dcf258e86e77d87bc079fe82000c0040014808d15ceb00eb64eb3ce86e7684780a77f694573b6a3c323c23521b9df61eaf1ea7a84006001c810d0dceb8cef88ec5428ae76167814767a34373b973d363c693a2b9ddf9eaf1e48a040080024").unwrap(),
    decode_hex("1C475378283520BBD3960036AA4FDC72BECE400C9D6650E870C5EB9B2E10764894C3A74EBCF3B5039D19E1C8804F0301E650A3278D80602AC6895C25E8194B1DA31381FE5EB43DF131BD82847869F4A9D11E3ED7B532968EDB61583664B6D0030C92183CCCA9E400180803256E5B88C80180803A6E85AA37500C0401D8E2CA5881C0602000602000602010180800183A0B1FD6BC20590018622429E301C703229384471980").unwrap(),
    decode_hex("B73D70AA7EC500142E1721118E31A4210AA9002F90362410941233818102801EB167E72BC018102803C8181028040602AC060200060200679F0EE52060200061887996E3AD4885F33F001BB80236843DBE12BBA9D5D00642B2A8D2A748A68CE0BD417998D596EB1594456E74F249B31E62745D52837D5AAF00").unwrap(),
    decode_hex("D2A649CC013C41DD36007F77C4CCB6D3AA308300651B4FDB0B1B4D79FBB383A62588B31C70AFAEA5E81301EB8E2ECCEF018102800C01810280087C000C75469F43A5DB4355040EE09B49EC58F1DC1D0701BF13A9141B44CC8E21819D96EA9BCA00C04001A955F7421D00602000").unwrap(),
    decode_hex("5E9373A9179F64F784159CD85165E6097C4B75AF002892536D222D77B80007034BC819010580602002ECEB1BCE02F03010003165044B0739B83A5B66CE1C1F6A645FA4030205007520003276157C6FD718BE2E70501D3E6D69FC2D8AB0D2977310AC20A4C78A0DBC04F1D3A75A3AB81D633D00301000378DF030205000377A21E20602000602000183A0BC675E6822F0331F34D9D1D842D14BF751989B03A2D9A6CF00107CFCF269D99B5E6880576B6BC136340F592D8006029C09B618E900C250D65415A5D6806F6BD03184D10D2C19CFFE91D4F83D49D24691860BA91482FC8E009C0030740D4610C7270000").unwrap(),
    decode_hex("08687FFFFFFF000C8EE0983A29D26634C016D580232D04858158A2F0006AD8275864B103010003236227FFD2625D935FA6BF72AB0AAB18FB8E701CE07675D128B09E001E1A8E9EB14608A06440C081403C00C0814009884D81A480A4C01808021809F6512382D004934D2201808015282305A0237ED3719D00752629A064031443DB103014E03014E1AC0C081400").unwrap(),
    ];

    for i in 0..spatems.len() {
        let _result: SPATEM = rasn::uper::decode(&spatems[i]).unwrap();
    }
}

#[test]
fn decodes_srem() {
    let binary_srem: Vec<u8> = decode_hex("02092d4ea63c70c132ae465700100000011020cf800007a780091563f0ec9c4f7de4383a0023f2060251902c00008008e000012de100002b0200002b1e0134899500").unwrap();

    let result: SREM = rasn::uper::decode(&binary_srem).unwrap();
    assert_eq!(4, result.srm.requester.transit_schedule.unwrap().0);
}

#[test]
fn decodes_ivim() {
    let binary_ivim: Vec<u8> = decode_hex("0206001210dc825000000000880558eaad5713d7a64ffffffe11dbba1f08c0e1110140750fe3540748ff1ac0abcff5e2bc307844402fff24112f01294570eaf081600002000408014e").unwrap();

    let result: IVIM = rasn::uper::decode(&binary_ivim).unwrap();
    if let IviContainer::Giv(giv) = result.ivi.optional.unwrap().0.last().unwrap() {
        if let Code::Iso14823(code) = giv
            .0
            .last()
            .unwrap()
            .road_sign_codes
            .0
            .last()
            .unwrap()
            .code
            .clone()
        {
            assert_eq!(78, code.pictogram_code.pictogram_category_code.serial_number);
        } else {
            panic!("Unexpected pictogram code!")
        }
    } else {
        panic!("Unexpected ivi container!")
    }
}

#[test]
fn decodes_partial_mapem_1() {
    let binary_mapem: Vec<u8> = decode_hex("020500003016080003094d8342fc9a94efb26b7193560c6e325cca0006138a08558f2236713c8327028a24482a480420000036d02fc0de2200b061c10fc05be4f4c3c7abc9878347f002c5a0000404405908084000002ceca22680460c05546623dd7dc2401242200000db40d500e280020ece061e011f102dbb76e1e6bdf820b068000102100642220000049dd5c0a611c6f4824e5f892011210000006da0787f1640010e407e83d47bd985efef062cd080c0582c0000818802a1100000024ef41efb09f57ca1034f5c9024d0800000367a06adaa00088f8e074646f7b108699bc487393f520b0b8000704100b6220000004b4ceae9781caeec43e35d292051a11000006ce08945d400109447b622467c818d51cf83137c02c16190000e0a2018c44400000969941cdd83eb1c407133cd241734021000002b38294ddd10055109c47da33483d8c2031ac0430b8810e1be70cd8501e058c4000383080c310108000005b4c62e50c08a887803900ec1770480ea84000002b68101e59d00040fede609af4a052f8d02eeabc1e8ce23e213b705616800080eb0c4000408040f604080000033f1f89610004102cc80a1002090410604080000033eec78dd00040fdb380a0f030a0411604080000033e6f59fd0004102cc80a12080b0412604080000033e3d499900040fdb380a11090c0413604080000033ee85fb100040ce3f40a14050d0414604080000036884de075000413240c0a13060e0").unwrap();

    let result: PartialMapem = rasn::uper::decode(&binary_mapem).unwrap();
    assert_eq!(
        2501,
        result
            .map
            .first_intersection
            .unwrap()
            .partial_intersection
            .id
            .id
            .0
    );
}

#[test]
fn decodes_partial_mapem_2() {
    let binary_mapem: Vec<u8> = decode_hex("0205e6e167b408000005566022a089a21b9bdf61f028200a00022000000657a94805e2bc69403895e21e0628af2f705dc55f429438148290000a4190000810090001100000002bfdabd7915ab517c24520d000069044000404034000440000000aff6b060258f2a983f148210002a4160001800110001100000012bd66c07095e4c202f0af0c502c8000a80008800000015f285f8acaf448edd6000c80008800000016101e02a4b0b9314380").unwrap();

    let result: PartialMapem = rasn::uper::decode(&binary_mapem).unwrap();
    assert_eq!(
        21862,
        result
            .map
            .first_intersection
            .unwrap()
            .partial_intersection
            .id
            .id
            .0
    );
}

#[test]
fn decodes_partial_spatem() {
    let binary_spatem: Vec<u8> = decode_hex("02040000301600384a6c1a17e4d4a77d935b8c9ab0637192e65000309c5040000ea71d4ff01000228ce2f944ff635b634871adb1aea1aea78010a1b8be2940a0d6d8d15c6b6c6c0c6c0de006280e2dfa68f635b634571adb1b031b0378020a1b8d8b8dc00da70d11c6d386d4c6d4de00a28ae35f236c4366034371b301bb71bad18030a038af798650d6d8d15c6b6c6c0c6c0c600e28ce35fc36ce366a34871b351b441b4478040a1b8d8b8dc00da70d15c6d386e3c6e146012280e2dfa5cd035b634571adb1b1c1b0d180").unwrap();

    let result: PartialSpatem = rasn::uper::decode(&binary_spatem).unwrap();
    assert_eq!(
        2501,
        result.spat.intersections.partial_spat_intersection.id.id.0
    );
}

#[test]
fn encodes_header_as_decodes() {
    let test_sequence = decode_hex("0202de140ce5").unwrap();
    let decoded: ItsPduHeader = rasn::uper::decode(&test_sequence).unwrap();
    assert_eq!(test_sequence, rasn::uper::encode(&decoded).unwrap())
}

#[test]
fn encodes_cams_as_decodes() {
    let test_cams = vec![
        decode_hex("0202de140ce5c7c0405ab23d82ce2781e9a278274bc633fa54587ca0a27e8302968a9733ff82001a103fe0143980106e0075801158ce0002f03adc08c4c800015781d620469633800abc0edb0239319c0055e075081185900002af03a0c0912c800016781c9e0565640000c3c0e0902dbb19c006de058d810218ce0035f0155c0006c67000df808d5fde662700073c0476fd67319c0058604137d31589c006dc").unwrap(),
        decode_hex("EE0959C86154DF864998CF44058677306E3AFB7CD5DB3477A6855C00").unwrap(),
        decode_hex("47FD7DC67AD4EC262E5545C4218FE5253C8EDDE55B675E541A8900").unwrap(),
        decode_hex("DC1A7FFFFFFF1B470846672DDAED9D8DFF29BEE7844C17B0E0418AD22EBA9A0C481CF2510ACBAF2E09A6911C9465B659F85EB746C0").unwrap(),
        decode_hex("94137FFFFFFF7ECE290A40FC2B08ED4C4DB3568617E432D52A63CE151638CD9E79BC54DE53267745CD774D9A741E3A5B5C91732526A17C").unwrap(),
        decode_hex("3DC57FFFFFFFD73D0E6AEC9FA06F946B00C04EC7001E6BFF8080").unwrap()
    ];
    for i in 0..test_cams.len() {
        let decoded: CAM = rasn::uper::decode(&test_cams[i]).unwrap();
        let encoded = rasn::uper::encode(&decoded).unwrap();
        assert_eq!(test_cams[i], encoded);
    }
}

#[test]
fn encodes_denms_as_decodes() {
    let test_denms = vec![
        decode_hex("0201e0fd1d37e7465aa8bc8006910d64c90404435932459d59019207135f2142be2be000186a09880050144018030014fb843f0a2fdd6bfb00c50a06022d297ff59fffe606805f9c00ff000d3124049be0064802ad89201387001bc0394c328112f801a5fed56144095fc00d2ffc93110015de00577f46580602c86ffd1bfccec7d80bb77fd59fcdb70e4289fbff7b008f33cc0577e00418010d86f0176f0019bfeeeca803a337fc66041e656810fdc002f0021317406d5e004a7ff1d8bf015170037bf862cdf013d78007a002164a001fdbffd2fff8b2aa0270dffb9800b58e700f9effb33fffec4b813d77ff2dffb362e80af9c000900c1b1240784dff12808119f003816ff7f401ccc828423f7ffadfffe602c0531c0278ff0ab1100671dffce7ff999fa01daefff440172cc880c7f8001a00ee63d80d0abffe0ffdcaec2093d5ff4b7fffd8970d1ab00533fd64c9b80c1d80145fe3b64640545c023cff922fee0883e0081801458e2013870052c06acb72015df800a1fff763880671c01d7015d31920577dfff58060d7c00106420480600c").unwrap(),
        decode_hex("0201000B3C198000059E0C80008000000000000000000005569D840713070C0FFFFFFE11DBBA1F051C0200").unwrap(),
        decode_hex("AD5857AA21920D3FFFFFFFEF38800FFFFFFFE003FFFFFFFD35FFFA1C405FFA4F94AC61DD271DA8613300D8").unwrap(),
        decode_hex("40CF7A0C004387BFFFFFFFC439800FFFFFFFE003FFFFFFFC2E9399847430CCAA56BC82CBCAE9D10B91DA489753CE92A0CA3AD082C3854477A3").unwrap(),
        decode_hex("34447FFFFFFFC93FFFFFFFB97A000FFFFFFFE003FFFFFFFB4793191190EEF338B9D9F6DEA590B9146BB84B56C10257C828848DD11831644008").unwrap(),
        decode_hex("A66E7FFFFFFF01BFFFFFFFAAF8000FFFFFFFE003FFFFFFF85FC0FF8780392A1F7754B7EB5E2C871B91317A22").unwrap(),
        decode_hex("520E5225F74FC33FFFFFFFE10E800FFFFFFFE003FFFFFFF93A2CFFD700AB3E74491121659AFA31D8D83A88BAD31F108FFC2877CD6E82BCFC85B7D29DB54E7F3F1593952C031B6B4000").unwrap()
    ];
    for i in 0..test_denms.len() {
        let decoded: DENM = rasn::uper::decode(&test_denms[i]).unwrap();
        let encoded = rasn::uper::encode(&decoded).unwrap();
        assert_eq!(test_denms[i], encoded);
    }
}

#[test]
fn encodes_mapems_as_decodes() {
    let test_mapems = vec![
        decode_hex("020500003016080003094d8342fc9a94efb26b7193560c6e325cca0006138a08558f2236713c8327028a24482a480420000036d02fc0de2200b061c10fc05be4f4c3c7abc9878347f002c5a0000404405908084000002ceca22680460c05546623dd7dc2401242200000db40d500e280020ece061e011f102dbb76e1e6bdf820b068000102100642220000049dd5c0a611c6f4824e5f892011210000006da0787f1640010e407e83d47bd985efef062cd080c0582c0000818802a1100000024ef41efb09f57ca1034f5c9024d0800000367a06adaa00088f8e074646f7b108699bc487393f520b0b8000704100b6220000004b4ceae9781caeec43e35d292051a11000006ce08945d400109447b622467c818d51cf83137c02c16190000e0a2018c44400000969941cdd83eb1c407133cd241734021000002b38294ddd10055109c47da33483d8c2031ac0430b8810e1be70cd8501e058c4000383080c310108000005b4c62e50c08a887803900ec1770480ea84000002b68101e59d00040fede609af4a052f8d02eeabc1e8ce23e213b705616800080eb0c4000408040f604080000033f1f89610004102cc80a1002090410604080000033eec78dd00040fdb380a0f030a0411604080000033e6f59fd0004102cc80a12080b0412604080000033e3d499900040fdb380a11090c0413604080000033ee85fb100040ce3f40a14050d0414604080000036884de075000413240c0a13060e0").unwrap(),
        decode_hex("020500003003080003094d8342fc9a94efb065719346d719b85cda000601ec18558fbbb8713d8403028a424817084400000b693c9ed41000407184435a81f0c161b0001402201b244400000169b79fb0034e3449207521100000ada4067e3e400109f898c235f9c709e5bc61783ffc334f8706966fb1586a0005812c450002c1090421088000016d310bd4ba000810014c6af63c902c46000280c40464888000012d4163f4d82bee6c12cd62481e684000000b6b5e223ad0004086076250c224058990000828809d910000004db5db8ba8045441dd2328e813704abd162b4208a8920d620108000011b5b593d6c8802a88787e0e17baefb22d5aac12320e8f6a08b4eae4004058510001830805210108000004b4c393eae141b87008160c4833884000002b6b82e775d1005510eddbb22c33b7e4f9de040957a2bfa7c11830e317ee8afe9e400405841000183880421100000005a54d9e970924e7c4802884400001368d0e6351000469f4c0089bc73e489ed60f02b02200040858e9000204880131100000005a0e0913d09050be203ac40420000002d07d495d008b1f89024d080000006d63ac4d4200088fb80ec160a8000414200ae4400000016b28a453c0b55904825684000003b6d99a33a110055107748cab70539812af5e51590008a88f754e121e8edfb03d1ec42c7b8fa0c938b0c6d8160b00012164808a8400000036a686514100043918ed02c1e0001430401d0880000002d1a64e1905027e32418c42000000db540d2fc08802a88b25b0917dcadafa20255eac75bb504544b551d42420b02800060d0414604080000036ce522dc100040eab4c0a150c0e0415604080000036d3ce39650004115cb40a140c0f0416604080000036b29e31e900040d5c680a1707100417604080000036bdfa2b05000412ab980a160711041b60408000003685e24ee900040db4860a1c0812041c6040800000368c364941000412537a0a1b081304224040800000368c325b7100040d43a00a230e1400232040800000369b9a63cd000412c460084880810000006d37e4c79200082598b4144a1e2a004a40810000006d176cb72a00081a774c10990102000000dab709e35400103b92e8289c4058009c8102000000dac119c9a40010448d1821420204000001b587b392a800208c1a90514868b801490204000001b56b53c6280020742570").unwrap(),
        decode_hex("020500003016080003094d8342fc9a94efb26b7193560c6e325cca0006138a08558f2236713c8327028a24482a480420000036d02fc0de2200b061c10fc05be4f4c3c7abc9878347f002c5a0000404405908084000002ceca22680460c05546623dd7dc2401242200000db40d500e280020ece061e011f102dbb76e1e6bdf820b068000102100642220000049dd5c0a611c6f4824e5f892011210000006da0787f1640010e407e83d47bd985efef062cd080c0582c0000818802a1100000024ef41efb09f57ca1034f5c9024d0800000367a06adaa00088f8e074646f7b108699bc487393f520b0b8000704100b6220000004b4ceae9781caeec43e35d292051a11000006ce08945d400109447b622467c818d51cf83137c02c16190000e0a2018c44400000969941cdd83eb1c407133cd241734021000002b38294ddd10055109c47da33483d8c2031ac0430b8810e1be70cd8501e058c4000383080c310108000005b4c62e50c08a887803900ec1770480ea84000002b68101e59d00040fede609af4a052f8d02eeabc1e8ce23e213b705616800080eb0c4000408040f604080000033f1f89610004102cc80a1002090410604080000033eec78dd00040fdb380a0f030a0411604080000033e6f59fd0004102cc80a12080b0412604080000033e3d499900040fdb380a11090c0413604080000033ee85fb100040ce3f40a14050d0414604080000036884de075000413240c0a13060e0").unwrap(),
        decode_hex("576D7DE039151870AE2541F0CAD9FE988369E852745C33E877A4FBDC80C11498500000C103C7EEE000C103D43B60040EA72CE30016384BF187A58ECDD9D2F3BB7EC5FEB81340C092303D214CB2B04F81BE5CA8030205001940C081401D37C44C39F6B5DD639539B469D6F819AFE5FC487DA451DB887469F0AF5D16924E9665A21109804A328220D5B3B76CC443975B0181B702580036069F5FB470086A014B0326A06040A00D406040A007CC5E0F13911234B31488178B013AE8185002E92C94DC015327C48018D83158DA9499A1A1658980ED5AC0B361860E0E06040A001D1BCA4142DCF011CCFBAC5ABE8AC808463BC37906201890038D0181028026304CD3625C682E763D50E108D5350DBF7C54C62156DBD8FD51BC03020500FE0302050000D3813CA40C081401E98946510414912A0DAE7D52CD5E42530A44300D7ADDCD0662C04C26C1A4C7018094778D4F91C58CA0C111B6818618202DAFDE1302A94B1C620E0185380002A4B5202BE38410428F2030205005068BCDA46DCE25F262A0D26AA0D717E6F4CC342853E94086648093D116C3499A540C08CD4126C79E33A7E006A58B870A7F40C474561956241FE8FBF2100337001CF8529D4DFD857E9901FC41AD11492504C62C00C081401A33030205008C0302050002030205000521F0310606040A00E480602003A0609A330B3D313A4DC3B2B66B35B328AA9CEC3800D44648D3C3D2C5280ECA06040A00").unwrap(),
        decode_hex("3BB87FFFFFFF441DAE0960A81E9E7BDE7813934180449FA86F7AACC2449FD7D3439D480C7703B2F405C030360ADB0E2C02745EF48D86645B6DB55631954106640189191020671DB5A4025ADEF250274861C821F02033271F42C20464E05002433203010003207042110000000000003684EA8AA0598A3B85C2C604E908DAFAAAC04F22A04540030850502E38BB489230411AE59F12E751514841E9458463509C657E0849E108184320302B8F1C743A6018A997027529DA572FAF602F9A8E137502C92C7175110039924A27CC031900C0D1DDA27640294038046094206CC609442E5CA48040FEE2455437C746D3F7CDEAD064EA2E2137F85AD99B0181B592280000215AD6FE9CF69380B50D390182890138B9718180800180800E2E45A0F100EEC0C74034360EE040300C41AC84AC2A4C273B1B043CB03006040A008E4F71AC00C2C00A8029FCB3F36B37114000C2400FE5676606A55F5880C3000C70CF133C10181028039800305382BE23A0002C33A26A9A1AF002C00D017A003F20AFF4C62E90CD1921121680EA441A05FF3A87A22C09BC81007F3E1086629036C0B8C90A0082DCF625944003D7499F990010AD58BBFF504910D66CE0").unwrap(),
        decode_hex("AFF94C85BEAF2D4960303E7E7934ECCDAF3481885341917A3EF26F4931E098018212A8D38070ED00606B3EAB8ABA2BC9D7C7861D5BE4419BA05E283058202DB52CFC0A658651886414F04C35030820604DDE2B5072A0030920E05711E3C6BC1E8016DD614038EEB82A57BE888CDA54D362A59E717846A197BDAE10A965975C42E2806410EC6C0F4CF40DCEA100304213DFB0015E27D297101691F068C2CADB1AB3F81812320564AEDB36B29680B075BBDF00630054C41A9CF108142D20DB147022007A018E31BC2E531E4D0F896244ECD58416D89025B8401811F24159009601987F742C1A0A99F8C6742D50DFF88855AB644669AE236066981A639C21689F397E0096BC0037CA63EB03F06401808024E03F93A4CEE84CA108F6FF1D6EEEB0B3748119A3A818700160832F18F467CFBE9CF261668019F005CC3B594740104A1405CCB60D768056E1AB206C28C0826500C88D8015889B016EFFE01C1013C5866B8147AD2318DA663043FFFFFFFFFFFC64CC4CD02F421006040A00B206040A0002B621940C09A8800C09A5013C140C04000C04C0").unwrap(),
        decode_hex("87635BE9934749B4239E508CEE2368591B08C41F69A7CC0060842B858000C5C2003F1F848254846D452299E0181B226C86758017401167B09D33DA1E150749619471E1755201A80C08140171F3F3C9A7666D7982A06485F2E9E3AA724A7E60583C400C56ACC93C092551F16405CE58CB3C600C4001D900C081400D506C96F4BDC194AA30099030205003940C08140009CC46A018080569201809502503014003013E970403014600C04000").unwrap(),
        decode_hex("8CF37FFFFFFF345635A129B08CFB437CD852B75677B0243201820C1F7CC063B0021C1D5FFA9316B8181BD40E0480A76F3BED1D1CA4D9485B030205001218B25E35AC65C26D89E5936FE1A0F39110CFA60D2E779015E64DE5E12C5094181F729242000654E50C096979CDC530022000000000003FD2CC0A4EF093C3DD8080ACF06E940B0A31B84B69808FD87FBB80FE903020500130311F40C05100C198420134063F15EE3CD806DA806226500A728448C084CDED6B341009BBB16000B707FF8A2177D1DB9F2819080435640A50291C111499D7263CD1BE119005035A0D4E04182C058181028034022018B0009D5C44E887E5C80417872206C4D83258DB76AABD826720128428D36F6AFEB7CE729CF41D78423827818102800A8F80A3EE7DCF0C7D81EC0AAC0C081403E80C081400B80C0814000AD78FACD689DE7CE773F5D7C4D4060830191C0006083EB2DC000608C9F0E20008E042159CED9CE32EBEAE6060476B0A22A84703F3723E008C91807A0119688764D91E40F5B8EB90F912C5E65DA99252A18A4030161A9BF0683517CC859008074729285800BB500ACF0206040A0001ACA014571A010EA5F3A4E251E7413EA0EF80").unwrap()
   ];
    for i in 0..test_mapems.len() {
        let decoded: MAPEM = rasn::uper::decode(&test_mapems[i]).unwrap();
        let encoded = rasn::uper::encode(&decoded).unwrap();
        let redecoded: MAPEM = rasn::uper::decode(&encoded).unwrap();
        assert_eq!(decoded, redecoded);
    }
}

#[test]
fn encodes_spatems_as_decodes() {
    let test_spatems = vec![
        decode_hex("02040000301600384a6c1a17e4d4a77d935b8c9ab0637192e65000309c5040000ea71d4ff01000228ce2f944ff635b634871adb1aea1aea78010a1b8be2940a0d6d8d15c6b6c6c0c6c0de006280e2dfa68f635b634571adb1b031b0378020a1b8d8b8dc00da70d11c6d386d4c6d4de00a28ae35f236c4366034371b301bb71bad18030a038af798650d6d8d15c6b6c6c0c6c0c600e28ce35fc36ce366a34871b351b441b4478040a1b8d8b8dc00da70d15c6d386e3c6e146012280e2dfa5cd035b634571adb1b1c1b0d180").unwrap(),
        decode_hex("020400122a80413376018800184e20102001337618a10320134373ae33afc3aed5a339d971e141e0ca50dcee20f230f1e128ce77a67b707b4891000400220334673ad43be23b01021b9d9c1e259db51119cee0cf5dcf258e86e77d87bc079fe82000c0040014808d15ceb00eb64eb3ce86e7684780a77f694573b6a3c323c23521b9df61eaf1ea7a84006001c810d0dceb8cef88ec5428ae76167814767a34373b973d363c693a2b9ddf9eaf1e48a040080024").unwrap(),
        decode_hex("1C475378283520BBD3960036AA4FDC72BECE400C9D6650E870C5EB9B2E10764894C3A74EBCF3B5039D19E1C8804F0301E650A3278D80602AC6895C25E8194B1DA31381FE5EB43DF131BD82847869F4A9D11E3ED7B532968EDB61583664B6D0030C92183CCCA9E400180803256E5B88C80180803A6E85AA37500C0401D8E2CA5881C0602000602000602010180800183A0B1FD6BC20590018622429E301C703229384471980").unwrap(),
        decode_hex("B73D70AA7EC500142E1721118E31A4210AA9002F90362410941233818102801EB167E72BC018102803C8181028040602AC060200060200679F0EE52060200061887996E3AD4885F33F001BB80236843DBE12BBA9D5D00642B2A8D2A748A68CE0BD417998D596EB1594456E74F249B31E62745D52837D5AAF00").unwrap(),
        decode_hex("D2A649CC013C41DD36007F77C4CCB6D3AA308300651B4FDB0B1B4D79FBB383A62588B31C70AFAEA5E81301EB8E2ECCEF018102800C01810280087C000C75469F43A5DB4355040EE09B49EC58F1DC1D0701BF13A9141B44CC8E21819D96EA9BCA00C04001A955F7421D00602000").unwrap(),
        decode_hex("5E9373A9179F64F784159CD85165E6097C4B75AF002892536D222D77B80007034BC819010580602002ECEB1BCE02F03010003165044B0739B83A5B66CE1C1F6A645FA4030205007520003276157C6FD718BE2E70501D3E6D69FC2D8AB0D2977310AC20A4C78A0DBC04F1D3A75A3AB81D633D00301000378DF030205000377A21E20602000602000183A0BC675E6822F0331F34D9D1D842D14BF751989B03A2D9A6CF00107CFCF269D99B5E6880576B6BC136340F592D8006029C09B618E900C250D65415A5D6806F6BD03184D10D2C19CFFE91D4F83D49D24691860BA91482FC8E009C0030740D4610C7270000").unwrap(),
        decode_hex("08687FFFFFFF000C8EE0983A29D26634C016D580232D04858158A2F0006AD8275864B103010003236227FFD2625D935FA6BF72AB0AAB18FB8E701CE07675D128B09E001E1A8E9EB14608A06440C081403C00C0814009884D81A480A4C01808021809F6512382D004934D2201808015282305A0237ED3719D00752629A064031443DB103014E03014E1AC0C081400").unwrap(),
    ];
    for i in 0..test_spatems.len() {
        let decoded: SPATEM = rasn::uper::decode(&test_spatems[i]).unwrap();
        let encoded = rasn::uper::encode(&decoded).unwrap();
        let redecoded: SPATEM = rasn::uper::decode(&encoded).unwrap();
        assert_eq!(decoded, redecoded);
    }
}
