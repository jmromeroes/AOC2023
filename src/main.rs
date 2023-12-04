use std::collections::{HashMap, HashSet};

use nom::{bytes::complete::{tag, take_while, take_while1, tag_no_case}, IResult, branch::alt, sequence::{preceded, delimited}, character::complete::{multispace0, alpha1, digit1}, multi::separated_list1};

fn main() {
    println!("Hello, world!");

    let input: &str = "9eightone
    hczsqfour3nxm5seven4
    9twopjqkghmbone
    rhrfthv886vflthreeztvzs
    tlbtwo62five
    ninetwonine234nvtlzxzczx
    28sevenseven
    2sevensxszqdhjg2threexzjj3
    2fvq
    781dk97eight26
    plfrsjtbl6
    sixglj13
    b3seven6817gjpcxseven
    3fivenlqcbszfoursixfive6sixfb
    zfxbzhczcx9eightwockk
    threedssqrlk2qnpkzpkdddt
    three67fourkbrlkf7mtbjprrth2
    seven3oneightp
    31three
    3894sevenfourfour
    3ghmqlnine
    7nine5zplh
    3three9
    ff6dhvzmdrgt
    3one3four
    fourvptdnbpqcxktwoone4oneone
    d6
    4kthx2
    ktjvrmdjf27five8one
    94gkvkghfjqpsix
    4gzstfpbqblqkxqrvd
    eight1nine
    8zgpptkqjdglpkssbpgzmn85
    sixpmfjrdmcj76
    six97
    ninesxs7
    one5lvxpfbnlfq
    jninepzpgtzq7four5
    fourvjjrttlvdtfour8qxdvlg22two
    7pqqdrrvcmvbr8nine57
    gvsdldqqxtjtsnnh147hxfour
    5nnph1three
    fkfzrdjvmnv9onemhlsjzrmxzzjfourjkvvgn
    ggxvcpfxlpjnbtmp3onecmgr
    three8rjm2
    88424four1
    jkgmcm7four63three
    9twotwo3
    skzzsfvhnine5dgzvdz
    eight71l1gthree
    shfrsdsgpsfpqgflvhdhsmlxvprqrplpmznfive3224
    2536sevensevenmqtrkzlfqkgp
    xnmfive8foursixsix8zjlczq
    7threekthree
    hcjeightone84qfjqkxxh9
    lnzkqfzmxonefourqsplvj6qthkx4fourpvrh
    76onejxthllvxrn82lddxx86
    5mdgxvpseveneightkxltknlz
    3sixnfourfourkdpfrgdsjhseventhree
    two72
    1781lcxvgz1sixrlxtdhgj
    three2kxrhnvkrsv9
    1bklbbkdh2sevenjkcckrkhm
    4txdqlj2qjjxxk
    zndcdgninethreekdspzsgf5ninetdx
    28hkvnlxeight
    rntmdthreeone7sixprqdtbsqs12
    fourqgbkxdj9eightthree
    eightntrlqffxzjjqrxvthree7eight
    threefourxtwo4six2
    nineeightptzqqqlfndtnq44dfhgnzbpjpkkjfkvseight
    three8three
    9mpjm
    twofive4xvxvhhfmqnfqtx3threetln
    8zpjkjmdjdclmlblsrqplh
    ninethree6seven9pqdbqll5xncgrvp
    ninebqczdbvshrsrlfiveonexjfdmxh1three
    lppfqnkgzpbc1nine6ninefninepgspdhvjtk
    7rbpjrhhrk
    nine6fourbztvbhpldcjs2vzvfxhfthsjlsvchfdftfive
    qthreegrq7sixvsrtztppnthree2five
    nvsvcds67seven6nine
    zgttjndqc34
    hdbr6jdknllngzdsevenpphrbdh
    jsnm7
    rmcbzk4sevennine12seven
    pdhzhfsvnbsbphcpfsbqceightmcqhxtgnine9six
    3three2eightsevenkrkhmsttxsix41
    5fournqxkbzthree
    one6four4
    2gfslqgszddk3crxtcv4njqn
    fourbzmcklqrtp3tgxcks
    vfsqvpjl7
    6zkmkm42hjrxlbjsixfoursixmbzsmm
    1mgmhtsjkfhgbrxcqlbfb
    5fiveeight8
    fourfourmcssix6two491
    6sixznqcronetrklkcmj
    ninefive1twothreenbjz6
    threeeightjmvvbbmpxkcb2six7
    hbgxxnthcrfndrncqseven2
    nine6twomlbkcgdfnjninetwoeight
    37
    878fivecxpdrbggkfktlghnnprghksz1
    pzpcpfour4nine3mxcfcscs8one
    cxklt1eight
    eightfivegxblrtcjgbdfourfsczgvm1pqpjz
    6eight2nbzbsqvdm9sixeight
    ftwone6nine686php
    16fivesxmpmkfzf
    vfmrtfhsrkpxkg897
    ksix77tzcmhbmnlqone
    hdpjfive4two7eightninexjmtxx
    sevenxcbbvccjtwo7rjdqmmtwossmqdz
    4791
    1dxddjrgeightonenlkrpffqtwofive
    qzjksevenninelsbvlczkdglgtlglcrfour6
    66smvxndcrb
    3mmthree
    291pbztnnmc
    oneeight32zzmsc
    sfnpmgh5
    4jtnlvgns4bpqbjteightfive
    fiveeightnine19mtxstlf
    sixrdzqlvndv71
    two2fourthreeqrtkvfqx
    nine5threeseven
    three6gt8vv1
    threecvrctmlsbgv8one48eighttwo
    none1hzlrptbxxh9clltwovnqxt
    37svfrkmzndz
    1snnmlmpgnine6gkxp9
    mxdspkhrgktmssk6
    vp3ninebqsevensevenpdlgldrrsixeight
    ninedvj637seven
    fivethree5eightseven6
    one6gzb9onesix8seven
    6fivexlnkmljfjfxpzljctwofour
    qsrkpdfourtmsmxcbq729lkkjndgvsv
    six6seven2seven6
    r9sstthc7kfhjfouronethree
    5threelfgmxk262
    pvfrx7one
    eightxnrmtjfcrzmflzjffone5htnine
    8three37psix5
    8nine1onetrhttvqfour393
    dbteightwobqbjzm4sldhhsix4
    3nphxlsfbjrstkzcsevenfivetbplknqmng
    fcctqjdtfgshjflnn82fivektzlvhm
    gdznbc758eight3twothree
    d9foureight6threencfgfivectl
    18xrgdprcvxstdonetwokllvznxffiveseven
    6vdtnrr
    5qmnmqbs89
    onellnghcl4
    six33
    9rfrqkjpbonelfqcpcvrqonesix
    four3brgjgdfs5ckccklxf147frfflvkvbf
    one6fourznsnzrdzql
    jkbzdjdnfh775
    1five1ssrmqpkpjsevenonertszt
    vrn3eight
    three3398threenksgf8two
    5jptbc
    9tfzqlqc7threemh71
    doneightfour1
    7nine9sevensixkcstwo
    eight42threefourvjfflgjpsix8
    nznsgzzbrpgphpfourzs72hfsixeight
    ninesix3
    rzp3kmjnbvjfbxrqftvjbckqxgh9fournine
    5srpnghlpgrleightwoc
    dfjsshvnqdbzttbmeighttwoonedvvnt4eight
    5eightcqtwoninehxmspsdglqkmttwo5
    ckhgxqvm6
    twoonec3threenine17
    bptqceightcvjxzngrsrhdvzvxhl8
    9sevenrkchssgltgthreetfthreeeight5
    5four1five4hqfournine
    81f9sevenjfgzjlbvzb9eight
    sixhscsfhtls58
    kbsrhhfjktwo6vksix6five
    f96xhv
    5seven7slxxbsjqktseven
    eighteightvvmdsknbfivegx82fvsnkkx
    2ztmbkmtmcdp4mnpfive2ckdlzk
    seven5prxngljdckzpdjttcjninenineseven1
    lxk24threekcftttdhg
    jjplbxkhxdninethzb12nine
    69gxmjk
    74vqnkxjmljtgdr
    six5mqtblgxddbkjczzv3
    nine98pflpm3qfrssixtwonttbg
    ktwoone61fourcnhl
    onekvvzbfvx6
    1gdsflnnssixrtnvlninefour9nxknine
    6vxdfblmtr32bzftmnsneightzbtcshp
    bpk1prxsj
    9fivefivercfd8
    svvlv3onepcrzrmmhvcjzxjlhznine
    zqffqvjdt5nine
    fqkhlcfhmmm4
    onetwo7
    nine37sevenninefive
    five3eight
    8onesctsrjrbfourfourxrckjkbsmhpsmlj1
    2foureightxcfnfs6
    2three9
    68onenjdjz3sevenfllzrxbmfonefvxrxs
    qkteightmss87khseightninetwo
    6tvpxrjtwo399eightone
    xsvdctmbhd3gtqxlkjkhf8
    qkxteight398vcbpgjz83
    npkqpeight79
    1lzqlvqzvvl
    ptk4dzfhhmxbqkeightninexsgjg3scgtjcxjfd
    5six8fivekrvkgsl7
    2s
    1322kth3
    9twoone4oneseven
    9fxnckqrmfive
    8xvkkzzkv48
    77one79seven2four
    ninetwo42
    vjdcxtpmbnineeight2
    ninetqsqdztthree96kkqzrqqsone
    brcbcqjdqzninetwodhxnbmppm1
    phsflcvsixqpgtqbpftz792tznzxfqnp
    goneight5eightfourbgjbl244
    8jpcvzxmchseven59eight65two
    vmjkgvnmbh63v
    gmvp7qhlsqqn5oneeight
    tgjprthree31fourdvs
    vkssixseven1mksmt3eight
    lqcfpbt2seventwo
    5zxjk1twopsrxpj1sixgjqbpzg
    vqp2threenktcrcnqreightfourtpqjrpnxeightnine
    318znqfivergd
    six92npqlvqpdmninethree2six
    dcsbjdfzthreelcrfmxzfjg18sevenqq
    eightsfdpdrp32two2zgkrmdbbrkdxbkpb
    8fournfngp9twotwo6three
    8nineplmkgqxkr373
    tkdctwo2bcknsclmm
    7threeeight83gnffnnvvmmfives8
    1cndhmeightjh8qnfhqnnbvbsrpvnbndqvxqd
    fivemddsffzmchtwo57
    two552ln9vrxzclqfckdzdblseight
    xbgtwone6hbhbhzqpvtt2jjjlcmbjrdeightnine
    1rshxbpeight3xhgb
    nine7slhsxqqs6threenine
    4one72b
    vqjdptqtvbksghfqsninesix9
    vgrdszz2gstfmxtt3
    1three6twoxqrhlrprvp47
    pjjhqxmmx199vdtsvkl3mmktp
    qrxqsqxklr6sixonextrbvsvbpxbqkmfmqh23
    5rvmcsfkgfour5hvrhfbcklr
    lfbhcszfjhdcmdnfgkjmlzggcxvqxmsznqhhmfourrqkhdzzqnn9twonezlp
    txp8cxftvdnb9three4
    3twothree
    9kqltlbdbv5jxfqh9mqsrfnccseven
    five74
    flqp55qconeeightthree
    twoeightsevenone9
    six627khhnzhjxddf4sixlspfmxtpx
    d3six
    314rcflrzpt6
    oneone7
    srbvrkhdz9sixtwo3fivefpttzvs9one
    9pfs
    697foursixgdfhdrxtwoninehgdznj
    five87
    5threeeight6fzrthree
    onecdtfnrljxdninethree4fivenine4qmcqtv
    2threeone92fzh
    fivesevenrn8
    1tjctrxfbdvtwonekkb
    jjcpppxztwo5
    eight4gbqxj4seven
    three4nine
    3nine2one
    eight94
    8xcseven8pjp36
    ninefour94
    5five2
    3ninek
    qndtpeight2
    kng8sevenpt
    6eightpthcstdvfchnszdlmkmtxndbptpm
    twofourthreesevennqcdnvmxgxhbfb6llqgnqn
    2gtbpdqj8ninec
    715threefour
    onebtshxmks8sixspzgdnfkrtljmg5
    jhpfgbvl1dccvsbzknteight5chccclvvczfiveddzx
    seveneightfour3three4one7six
    75rzsxmnhcn
    xh2
    dsvqrsix1nine
    6nfknvbxlfzbn7sixsix
    psjspjfp7fbhzldkjrjzgkmjqzpc
    trzpxxtpxb5onethqjpxgghbkbjqrrjtninetwo
    threeghgttkkjqjblld8mlngjtdzone
    grmjf4fivesix
    xlxfzcrvfvfkjvjprstlxrxndjx7kr2
    ldnspmtwo6bnxrvggp8pjgktbtmjc3
    gvj32
    sixnptnlsmxxrfive48vmrmseven
    qrmc1rgmck
    three3jpvqpsxqfour
    one8dkdhplflcl83twovpkfmzh
    beightwoxtjkxzfcqj9four
    fivetwo25seven9stznfdjng
    8nine6
    seven1six7
    69hkzgmfljnx9three
    sevensevenqgoneblnq9
    79one3mqnmonexhnkvphbgcvjzzrvlmh
    threezqnjvrpxhssklcpltwofoursix4seven
    53jntjrfdbf7tmtjmsqhxsix
    5rdfnnmnltwo
    35gzm
    hbgdptzxd55threenljvkcreighteightnbrlhgdtshtqvtzd
    46onedpxdfgjpl
    prl91917
    sixbgfjvnqhcqninetwo1one8seven
    tkcvr56hh
    bxqcqlsevenrzgnrfzz2grfqh2dgzlz
    fiveonevlhmmkrcsmzfdb5lbxrfivetwo
    nineseven4
    threebrgzvzhbk8cbrkkpmgv2
    clzkgsxrmnsqvvhfd55seven9three
    nplbth9cfzjqd6ggone
    9db
    rbnqzbsmdsixrmnqqdgmx1hx
    3xtwo9nine62
    6fvhbdfgfjsldtlvln4five6
    5jc5
    sevenjponethree8seven
    eight92rfh4nine1jtwooneightb
    qtzninetrnc32three3six
    fourhhlztg3foursix9stv
    seventqqgj81vvtc1five
    8nineoneightg
    twomtwo5rncmsr
    fournine7rhggqjthreetwoseven
    eight6qgnlxjssqqdlv1tlsxcpdbffxmjpptcpcdpzxzlqqtk
    ggdvhdfive6
    4bdfmbqthreeldmthreetwolntnbgvlldvszplfxpsdslqlh
    nine1five
    38fkd8z
    5twonmp5fivepmrkfhcgxbcpbjksxqvseven
    mjxjzfour3cdhmzbgvqtqxtdfivethree7cb
    nine9xrpsqhtftwothreesevennncsdlclttd
    6bfndjktxdtbgsgcncqxrqdrs3
    xvcgbgsevenfive6dpceight
    jhzonekvqsmtpsgbsrjs8six
    lbxlcdzptwothree5199threepxln
    two1rktj1eightqppcfdlm2
    336
    446four6seven
    threefiveone3hdxkgvdvtwo
    2klvktzjps4eightninehhmcdkrbtddclnfour
    fivethreejqsjhfd1
    qtgscf1424sevenfour
    eight2791sdvjbbt
    4one1qhlglzzpkkvtkfivegrjbhpz11
    eighttwodgqrlsfive24jldfhpx
    518three73jxfour
    four1six5sevennjcxj3
    1k
    8seven29
    7xpgfourcskxhftsfnzzr
    z16onethreenine
    phtqgpdkqzd8
    9tseventhree8tbkbpkzlcs
    7nhh7four
    7three1xrtcrxpnmk6djzktl
    489onenpbtghbv7
    6eightwogd
    2eight4hmjzchninejqjnnrrg
    kvpffsbghsnzzzlbhkkh3nine
    ninethreefour1nld
    1four36cjsscrbnv6qnlfivefhrd
    3sixone
    158hnljkbtrdx
    fseventwoeight64
    threeone3tmjntbxzninefives
    cqkcltng2nnine5sbmqfj
    seven6two1nzbonezn
    76onetkgbgdnnff
    43threes
    lkgncsjkgklnslrmvsevenseven6nine
    317ps7clxzs
    zmfoneightvxbchjhrzmqvxmkkbfgxnine11fnshxqrcqlkrfb
    6sixnine3
    1qttpkjghbr3nine
    six6fourfour
    eightqfpdlppsjpzjfive7
    67sevenlrqkzcsfvmrsnjpnkt2nxnsfvbn
    threethreetnmhkdfive9sfxzdjcdm
    nineone33tmgbcflbkgcnjdxk
    seven9lmjfbfvcjlfskrm5sevenfour
    tx4one8qsrxkxztpseven97
    mgsdjkntwopnvzrpmhgtwoxpskmlnsd7
    588
    4sevenmjqvqnrsixtncnfhlkmppqlxvhzhjkgd9eight
    5jgqvm
    onefivefour92
    xjbkceight3ctkdndmsh91
    2fcncvrvqfb2bkzgnt8seven7
    nggzzfgqfjrdbsjz7
    4nine5two5five6
    fivefivebzldqdgfsixeightfoursixthree7
    2xqxgg2three8vpvfcphxvthreegprqsjlfeightwogd
    nine63
    qoneightlndfive3nine
    lbcnltpbgthreefiveb8
    lqvrsjdgnseven83
    7shbfourffjgvpqxeight
    3rdxmr4vk7
    36three
    1qmvvhqpthtlzq1gmqtqm
    dvfctxszmqkpzj2812
    5rnzcjpcxgc2four
    twonineone5zbxsrcgndfour
    86threeonefourninezcfghkqthrrh
    5two1gnhsgzhvbb
    xsix1
    xgvjqqqggsix6four
    fiveonenineone4drmvcg6
    479bdhgzh6
    gkfrmlpc4fivefive3onetwo
    7xk355four1one8
    fivefivethreezxl1pxvxsfktvrseven6
    one8qpvchhggcfmthreeone2
    two575ph1
    four5jmcpmnvsvnsevennntmj
    8dcrbfs
    trbgp3dphtwoninetpnqrlhqq
    twogchrpfourthree6one2two1
    1eighttfcpxqqvndzmhrj61s2
    rdvbmsevennine56twothree93
    54xczjhghc
    cjlpbqlhfouronebrrmtxlcqprnbfn9
    tdcjgcdgfive82nine
    seven5eightpgmjzkmq1fiveeight
    two7qttwo
    rltbflkbhthreeeight1sevenklbntffk
    four85fivethreehxjgqrvm66kmjvljtd
    kbm3
    stdfsixpgtsh7qfvss7
    hvlgddtb3threezpfdeight4fourqbsxbbmone
    sixxqcdxxnlt4
    d659ninejtdjhmszl
    x8kloneeight1
    6c6
    threeone8fourl8oneseventwo
    eightdlhztztmpnzrseveneightltlczb1dkssbntzrnqbqtsskk
    hnrvdtwoh41rxppklxhqdqxd
    65xxzttvlvcb
    6ninexvxddmcr
    eighttwo9two2nrcfnfqddmmthc
    nine5jgksevensix9
    twothreebgrtdknine47
    hpsvckbfourseven722plpqvgrr
    kdqtskqp8bpfjgr2sevenzlhqcqfcchpm
    49threespvsqhcbkgfkptplseight4four3
    2lzj48four6nineonetwo
    7dfiverkr77sevenvfhrqvnr
    three7five5
    lgzmgjnr4
    seven4sbqronefivepptqkqbvntwohttvklqkkmzlv
    8sixtwo
    ctn4fcdxngvvbthreertnbncskgj
    8eightsix3
    two56x
    3gmlspbcdhs
    6ponesixmtsxmqctbv2kkpp
    three6two496
    1mkqhdbsdktwothree5
    bvp626v
    oneonerfgbtgf5
    8r7twotwo
    1drgmbtsc
    qsqb6pfxxvrbnhc7sevenzdzrtkzhjmchnrbzksmkrvcx
    hcfj8oneightxg
    seven5mgbkgdttnsixmncdtsfchsd4fdhvbpxtq
    sbqttjhfds8vcjmgsixfonesixnine
    twomggrdbvn4nineqbnqkdhgklcqtzh
    63four3nclcxrhdzrjpnb2
    9168
    fivejzzdhd3
    twothreekfddrbtk2fivem
    7bscsjdpfjchsxkrshjcrzcznine37eightnddl
    two16shfjsixglgvkjtxkvdlqtwotwo
    six6q
    fiveonethreefourgvtdhf259
    dhgnmprvmx21one
    4ninejpzpsgkskbkcfive25
    three2mksq
    7fgxztnxnlrsht6seven3ss4
    9ntqfq22eight758
    sevenlqrvsixninexzpx4
    one32rznklfb
    five1fourtwo9seven73
    3hhbfxone2eighttwo6pfsg
    nineonetwo9zsprntffive4gtn
    btsixeightone2xvnsix63
    4rlt3
    qfnhxxzhlninefivefour4rvnbdbzggqninedzshxnv
    gdqjzdxvs3threehhkm
    vnn185nine
    79sixsixfivethreevzskfnpspninesix
    two1611
    threethree1mqnfpfnsm1
    kdkxlxtqzqpsxtfj6svt
    1nptdgtdpct63vjtpxqf
    2four1eight
    sixseven2hcnjzbdfk
    6threembsdhhtcb9ndqjrgktf
    fourg9sixmgb5sixsix
    7six1xtwosixninenine
    rcvrmldhveight76
    twofbmfczz2fourkfgjcfqst5three
    fivesix29qltppptworflvjfggcvrgkfqgmtqjpsh
    seven4mdhpflzkkfznmlrc
    sevenqtgfpcznnxslptdtwosixzkplmzqbxxll7svgxqjlhlv
    qrssxmxsxsixone38hcqnb727
    lrfour9bdlfjcsonefourk
    83fiveseven
    28qnine4kbeight
    seven8five22two
    twotpxzbbmztone2ncr2tsixone
    eightcsjrqsscfour5
    8rjdsq4lsix3
    bclxzxxqbv7ldgdtq2
    four5xfpztvttd
    ninen7flpbfzpone63twohzj
    five721
    6gqcxpb
    58kphone
    five1oneone3rgzhmdbdgqfmvm
    jfsphkspl2k2gflslvtwov
    889nbcbhgdh48
    vlgsixsevenddzxeight7fsc4
    71ninethree7six
    zbvgthreefivefivelpp5
    nkpqfsh8fivethree392
    two6cshbcj97j4lqgjvgl3
    sixone2
    oneeighthscfdlgsg8
    ptlkdqs94bnbtzxpqpqzg
    znf8three
    eight5qkckqbjdqsxsixpvphcqbvfxsix
    fcvbjnvninezsjxfxmplpbbdkdxv2r
    vlgbhsjleightkjlgsix4six
    htwone7htwotrstvbvnldgct
    eightrshrg743998
    3nsrtlmhmz9qzbqkrpnh1fourfive46
    onen8
    fourrbjgknhnthree7
    zb79three3qslrsglbrpsjrqmdrmrlone
    hhlqztk22qczrcqnxrtfourtwo3oneightsck
    jnrqxrclgn7
    5ninelfqcxfkfive
    6fivemtqcmcqbtwo6
    onekdxzcdbjgzkhnhsjnjvxgx3fivesixseven
    nhqcx14fourfive3
    45kljxlbzdkbfour96fourone
    5975r5txqqltj
    vhv88
    6onet25five
    4two72nineeight
    2foursevenonegcdqz
    353v7sixsevensixeightwobk
    vsqsxgqxn8xkxftpmtrtssxgnfqcqdnsixdsxhhxgonefive
    3four716seven6xmrmhv
    one4553two
    hqlpmone4rvj
    pp7mpsrrxcnkseven3twop
    nine8bfkgjkhjqc8ggponefivemkeight
    fournlh9sxbvjqmfourtwo
    cncvbjvzmcthreesix3d
    five8vlsix5qfkfdhjcghcvgz99
    tvxsrmnpsx3rkmxfqg6thkrtbmt
    6nine5threeeightsixgvxrmbb8
    mgvxxkkxltwobgghk6
    fkqghfour896bfpvshmsjrbpsz
    5twoprnfh3fvsrzgcdqv72
    5ninekbf
    tlcmtb78
    smkqlqxrglktzn24four
    onengbqghkzxlmbjltndnrq3fouronefourz
    bthreetwooneeightdsm1
    hfdskdkqtcsthxcfheight3five
    41two
    tfourtnmxjsmzclfmbmhjrjln57five
    p6six
    onethree855
    sixccdpqbkbbxdxcckbvtwo8rsixfdntfqseven
    one4863one8
    nine29fourfourqljfour
    61bshj
    eight7zmtgeight7gkrkdjc
    pgcxxjfourrbtzgmphl83
    vbb426seven
    7two47ztfhll
    5five4
    ninesix64nine
    7dtvvlkq9threesevenzlfznvninezxpk
    5threebgsv
    59threeninethreeccb
    295fivegdsgnsixsevenfive
    eighthqrxpsvksixgn1
    fiveeight8sixfivelt
    threenine2
    zgjlcxttjkjd5threefivefive7
    fivetwo444six
    nbcsc6eightsvflh
    tsdoneightnjbvfktxvvc2
    zhlctqxqxfp8two1eightrnqsq
    82kxfsxnvfour
    six2cpvmz4three
    twollqjbjs366
    seven7threesjlbcznntzkfljxqsfoureight96
    dxsnlhveightnmxone6hzgntkqbfhkdvxg6
    five8fjfzzntklkdgbplpnvdcpdcd2btnphjnbm2
    1six5drpzsx933one
    bqffgcnvp5
    pbjktxtslbbdgq6ninetwobrvbbzhrtsevenc7
    9kprctdnpzone56three
    twogbdvsdvzp1
    fnczdrvgzngt9
    three56gsljqzxgnzqpvh17threecvtgpshf
    ghlksix3fourfiveeight
    one95twotwoonet
    twoonesixnmfvtqsd8sixfour8p
    eight6twojzd
    6fourtwodbzrdghsgngljtj
    seven971nbrzgmnine5six
    lxskt34sixz271eight
    hv7one4five1xfdzxgbbsgml
    sevenfourxtccpbxfcnnpxxl7fiveone
    onerpvsvrgcn8kclkdgqjlsixfive
    mtthreeclxhfivep8threelh
    9sevensevenqxhhkonefour4four4
    fivesixsixmxnxg5pxjd
    zdkzqtjgsevenseven5zkqslxpll
    4sbbltdfvskrnxmlj8
    7fktr
    mmdhfmchmb1bqdtsmqzrlqxr
    3fbzrj5five
    threetwotwonine8
    one7bzxkqkflmksixnine9fourtwo6
    pdcfjzttllhsix3
    51fivercdkdrnine8vhckbxsrvg
    rskjjchgone71frdxpsqpmxfrsggmsjspklthree
    s6eighteight
    j1eight
    bjxznqjvneightff5twofjjxtrjlm
    fqsdttjbsmjf24cxseven6
    seven1srptcft
    tdqeightwo9rcncnqgfoursevenpzknnc4
    4eightone2mdfp
    gdxgnnfl3fzxqnvntzpzfcxtwoseven
    45khpg2gtnplgrdc28seven6
    twoxcspzzmhj77two
    three1blbrqddthree
    eight6eightthree
    8nine4
    eight82six85nine3
    6eightthree5
    sjplthreev6rqqone
    rgzdkzbsnvhd33six5
    vxdkzjprsxf2zpblslcpr
    vzfourseventhree7
    five72
    7ssbcsmt4nhbczngjmvdlxrdmkjxxzcczf5
    eightrbtvglxstscklzbfive65eightseven
    threethreeqggdxdqrfgpl542ngcvzsvgn
    qq7llbmzf6five5one7
    5zhnkvdfzlfpqgfvdpqrcdtvkp
    3eightthreeninethreeoneightf
    one8six52eighttwoseven
    93ggfqxzjzksjtwoonethreefive
    7twothreeone
    zmkrnqfgpvlfknseven555oneightrgp
    jhpkdgppzthreeonetwo6
    qcxtwo1onefivetwo
    one3tzsnfive3one
    cnbhxfour9kttkpgtr
    7bvgbgnlkkhknkghtdpnfourtwonetxb
    5nine5
    91one14three
    48sevenpqvxdz
    fivepmlhzfnmr8sdqhzrdkxeight763
    xtjpzzfn4dshfdqfzpcthreenjnine
    nbtdqthreexjsg9
    nkgthgfjm583
    ssixtwo1
    155fiverhvj
    onesevennine9
    six9rfjqlglsevenoneseven
    8jxktrjzqhhmbtmj
    qfpfcl7six
    7threelqxspxrl3vcjfltpskxsix5
    1seven1xnltmssnskqxqvjrpsdrqxls
    seven9sixninedldsrp8
    twolpfb6
    sixsixmdxvkmnzthreekhftctrvlzvfqthree8
    one1sixfive7
    vd3xxvhvszpnine
    75bp63
    six55onetbnxg4
    5five2twoone
    threemvptgnhxr69nine5
    2fivepgzbmgdhtgcjh
    glsbcqm4six1njjbkslthreeseventhree
    4onetwoccjtjqfmb
    threetwoonenine6346sjbpznqfr
    zb599sevennine
    sevenone988
    xkgssdsdftzsevenfour144qvszn
    nine9sixthreesevensix1threejfggcrbc
    pscvksl685
    1gpdvtwotwo
    mlvntk4
    onelblcmxhklxcfxm1
    4onexrngzrfive
    eight9nine8
    tbdcd4bmxftnlvc
    ninesevenrdpj13
    vxdhgcfdkvgztbhxqh3fmsrnkjheight
    sbjpgsjtmgninexkv4
    fourldgztjqnzlktxhmncdnfmdlzzsrrtbfhtc1
    jgmzmvqmrbdfjrtzbtzgxbcbq2ctvxjstzsixnine5
    666six9
    6gxhfive9xxthree5pqf
    four394
    lzqone8t95
    2cqhpgmszthreeddkkmleighttwofoursix5
    9sevensixsevendhb869
    2foursix
    fivetwobqlbgttxkqkmzxfn6fivejbv1
    8fivesteighttwonine
    nine83fkjmmnrzgjtdsmrqtktfvcsevennrqctstwo
    eight7gx
    jbdpffour68eightfoursix5
    ninesevenmnzmxpxcjjl2one3
    fourtjvtmbxlqnjdrjmgrzrh7rfjtkeight9
    three3twonine32
    fcfourfourmgtsv5
    42foureight8grczvgbg
    5eight7fiveeight77five
    seven6threeqrvtg
    eight2ztmqdjhjtqxhvfiveeight
    3dx4eight8nineone
    fivethree5drcmbgmntxqqf8pdhtfh
    eightdfphzlptxsix81seven
    4qhsvhplpn9smqnfgd41
    z18ls7
    threesix8bseven4
    19five57threeseven
    ctqfqseven8
    15six88
    oneppbxscshpm2nine6xljmsfpqqcsixeight
    one5threexntbrf
    nineone6qll2jfhdtvvnrhsgdscxb
    87vjlvndc8svdkn
    sixlqnfpc2prglgeightfour54eightwoll
    6btprcqftmthree5xxntlgrcone5
    blzmjpqkcrfour84xkhqk8kdcrjcksnb
    5fivetwothree685sevenkfloneightj
    bdthrfmz87four342eight
    7zbtbdmfbxgsevenfdlleightnfddrld4
    sixpsixpkvtxgsgbz35
    17fourbjf
    6chsffs6fddcftznsix1
    77two16nineeightfour
    8nvhfv3two
    nkdtwone8gcnjnfgsxtjjmt
    onethree79tjlnvgqf
    276lxfqrrsxjqbbllthree8
    hhdtmpdpdztcp4threehchrvhtxvbssgpvqk
    nine8fivevncfslncdn665
    cltgddlcqt83bnktpsixfour
    9lssevenqznhgqvvpp5sjlninebkkpmgmkk
    5khrnfzfkkcqnmpfour
    8ninerf7rnrh
    3hdpvpbrtn
    522onehvzjv
    7xvhxqcpgd
    82rxljgfour6
    gztjs8g5
    mhbtwoneznkzfztm2twooneninethreextttcheightnine
    9vrlllbqcnpb7drtcxcpsbz4
    73hqmcgkc273
    mqmc1
    foursix31eightsix
    sixeighthkpxg5pqqpfbfcmhjskqqkbtqzsix
    4pthsjjkfsr11ghfp56two
    threeninebsvtkcrn7eightwos
    1bbfmrf67four88four
    3vh
    three851two3qrtpqseight2
    four56sksixone1nineoneightvtc
    six349eight5jtxdc8
    ntvjdm1rhxbqdgsbqpvbdb
    21five4six
    fivefgnnlbrdjsix1two
    nine3eightthreenineseven
    75rmqtnqrgnmeight
    four5rfgldltfkhtjqjfbxfhhmdd9fourbxvpdcgx
    onesixtwoqcbqndpgt72three
    2xtsgslrgrf1
    psmhzczsonenine8fznbt6stvcsszlrsccfmgcpf
    38sbpzgfvjtwo3
    two1threecjmgjsm
    nine19
    nine2lchndsevensix99dh
    qbknlrd5rsone
    264gzqrvckzvnsevenxdj
    6jzppeightgcljcnnkgxhbmbv2gsrzszbdrmvzdxhktjmdr
    426twoc2twothree5
    sevenfour3sixsixqncc
    zfjtprrs89
    99threedbknkl
    vrpfjvveightgd4gpvnpdbp46three7
    8ninejttx
    gxdmgseveneightgncxrxdtx9threeseven
    two9fivenine283xxtwo
    four5four91vbqvfhggzjd
    9seventhreegrkm9vb2rmvhseven
    8gqftwonexp
    fourqnvdshvfive7
    dstmqp6hgzqldhxkncjbxh
    31nine
    nine84fourpnhxltmb7onesix
    19pseven7four
    foneightfourfour8cvbl
    four9qldqps
    4tgrjglvtgghvkhtttwossgrfbvlpbxlhfmq
    7fouronehjdmx
    6ninevvrfqntzmjxpc
    ninedfmxcnhth1279one
    7gzjmbgonebjxmrvhjxrone
    fivelpsstxjllxfive5sevenkbqmgbn
    2three81zxvtvnjqfsb
    four9threesixzlknkxz8one1pvxff
    seven33219
    dlnvxppbzrlczfqrcbzcvfour7seventgnn
    26tzzcmlpxfour1rq5jzmssgxhvrzbk
    1threekjvfnxvrk7bhljthreenine1
    98nine
    9one56six5seven5
    27three18
    7smbbsvxbvs
    717one5eightninegnine
    fcpnvdkp6twonez
    85dgdlfive
    pmtzmsjblninevpvcsz9mq75
    4ninepnsdmjkx3four7three
    563nine
    oneoneeight8twomrxprgdtxm7eight
    1fivefivekkqhrzqjjklq
    6hhthnkdonehlrvc81three
    jtwonetwothree5znqsvfour5czgsqvvtgg
    fcfskbfive9eight
    221frgs5nineeightwojj
    jdmgmsglmpl3
    479
    fltqdrksdzvdmkfive86
    ssnnvxfourthree5
    95jcfrtfbr
    56941ninethree
    rsfeightwo8seven78kmvdzbrmthree
    xpjqjknchcv8twojzone5sevenone
    61sixninekzkgnlmd
    41gpbxsk8qz
    four5pjsix5bdfbsixone8
    fiveeightlpghqqnjdfivetwo7vtgkjtwo6
    sevenninexxvjnvvhqfiveeight46qtfour
    8znvhldrsdfqpnhccthree1
    fvoneglhqtvfbr3four
    eightdhcngdmvnklgsix47
    6zcqb
    bkkvhbvgsixgdgckjzsj5
    gtjckhq73495fq3
    rgrrkksdcftpkkrpssgfjtv7twonine
    sixsix31
    5mfd3kjvhmvbhfive9pb2
    8six6xqvvxtbmqqninefourlpzmmhdsk4
    two3sixxtjrl8
    1jhtknlffcjr27
    1fivebnvbqt6
    8ninerdphtvbk11sixrkzszcdngt
    sixsevenfour53sixbqhcjcthree
    dcpninebpbxdrdtqpm933six
    1eightsix4659rtgbcgb
    two84pslqtrslbxqnksxcxhqrcmonencxqffmt
    ftzzgbzvvbj562cffcvncsbhrzzv
    two6gmjd
    jlfgq31four9seven194
    dzeightwotwofour39eightcgthzhp
    6sbgbqbb5onegxkrcfiveninephshfrftwo
    46lgnjvfjt9vjtqmjl18
    mcrsjvfive2
    45gbrpfgttwo5threejlksix
    1rpmkjxsdeight7hnpjjkthree
    9vvcchtxdltmfnxtqmp1mdntlj
    eight8mkzgsdrdhthree6four
    33lfivepcthfgfivethreeffftjmgsxl
    9bhmz8btcklgsixjdfxlfour
    vchvmnn7pk
    sevensix4
    3fourthree4seven
    oneeightfournine7
    p1one8zcsd1seven9
    32t1fhjzbkmfrgmtwofive6
    1fivelxjmvstxnblxlqdxvmbqcpdzzr7eightone
    mktoneightskrgbvmptm89cxv3
    geightwodd88ctqzfourfivesix1six
    one6seven2n
    9nine8dngqbrp
    h5dtnqzhpqjnnrxxcrjc7nineeight
    four2prxrhrcvfour
    2onexdnj7
    7xdnndpzsbplfrfxdglrsqrc4
    chbmstkhdfdmqsevenone31
    fourthree99
    44fflxqlxjr9pcsthrjt9
    7rdfour24sixbk8
    vnhfzbr4nine2qgoneseven
    5h8ntz8jscjpkhg2
    tcsz9ninelbfsxppbjpp8tbxctrtpfz
    three4nineeightgfl715
    1tdhxlqbkx69ninenine8
    46ninerjqfivenhcdgprl
    199mhhsixdplthqpthree7
    eightfourxjjgrhmdmddzclgbseven9nine
    19three
    ckqzd2twomdxz
    7jxtpmnhtwo6
    tgclgnineone4qqcjgsix4
    fourjvxcmqpvxtsevenznlgdx13twohghrxdp
    sjtp5nine4gcrdxscxxtgvscnthree7
    3two5czdn1sixbpzzsfhdhqfour
    xflzgzcm7ninepcvfpmssixtbjzb
    7zmlcpsjneight7pbtqbkgl
    sixdsvgjqfseven7czsthree
    twopgmndmtzzsmxcjptg4sqslhkseight9one
    2threexgbtcjp
    twonfknjvlks5mcpmjcqjfsnfbfivetvzprlx22
    3threefoursevenrkqfrxgx
    fiveqpstwo4rnxd75fjgpv
    82sglxctseven
    fourmxnrhzx8xkzvgtrdlzbzmlhfbf1
    1fivejqvrtl47tshfgrc3
    97twoonerntrscpb
    2ljplbrsgrjnlktngjd
    854rmtrkhjzrx2nine9ldqrqq
    onejmrcmphzksixfbbftwo7
    sevenkncjddrlsixzmb94one
    twolgqcnh6
    6oneightsr
    jjl4seven
    23eight4
    8eight277kts7
    8937
    9cbphfncslbvn23dzcpz4pqzlgfjl8
    zqkhnb7hm7grdpnq
    blczr9nineseven
    2mxzhtxbjjq56onexrssstc71
    vkvmmgreightgdbqq6six
    1six8sixsevengnqbqgxtwofivem
    741tfbvpnfour1sgone
    glvctfourgmlrqbpsevenvksevensix9
    49four
    six9five2sevenrone
    fournvrctlkztwosixsevensixvrz1one
    ffjl5gztldndtqnb5vjp
    bjckqhbnthreethreeonervtkdvxkgf43
    mhrckkcgqdms1rvrfcvpsn3trmfltvbhr4sixlpslr
    tffxs4sevenzsdxz
    onetgj2mtrdqnsixfzvcscfourn
    3jsdxk
    flhmdp6eighteightmcxcvffive
    bxbzngmpds28
    onexlqp3bhh
    jlsjbs2
    1twoonefivenvvhjf
    864two7ninejzmpzp
    eightthreeseven2nnkvlzxkvhszfpqzhl37ddqvnxg
    xkkkskcvsscmmgc2ninephntx6
    lpklkskgcsr8eightsbxcjx
    4threelfvzndfive
    nx9ninekvzzdlncblkdqbgspdfkcx
    tfn5kx6twojmzgbdznc2
    5bszzkpcdxqkvkf7tgcone2";

    // let total = day_one_two(input);

    // day_two_one(12, 13, 14);
    // day_two_two();
    // day_three_one();
    day_three_two();
    //println!("{}", total);
}

fn day_one_two(input: &str) -> i32 {

    let lines = input.split("\n");
    let mut total: i32 = 0;

    for line in lines {
        let mut results: Vec<char> = vec![];
        let mut i: i32 = 0;

        while i < line.len() as i32{
            find_word(line.chars().collect(), &mut i , &mut results);
            i += 1;
        }

        if results.len() > 0 {
            let n: String = vec![results[0], results[results.len() - 1]].iter().collect();
            println!("{} {:?}", n, results);
            total += n.parse::<i32>().unwrap();
        }
    }

    return total;
}

fn day_one_one(input: &str) -> i32 {
    let lines = input.split("\n");
    let mut total = 0;

    for line in lines {
        let line_val: Vec<char> = line.chars().filter(|c: &char| c > &'0' && c <= &'9').collect();
        if line_val.len() > 0 {
            let char_str: String = vec![line_val[0], line_val[line_val.len() - 1]].iter().collect();
            total += char_str.parse::<i32>().unwrap();
        }
    }

    return total;
}

fn find_word(line: Vec<char>, index: &mut i32, results: &mut Vec<char>) {
    let mut i = *index as usize;

    if i < line.len() {
        let ci = line[i];

        if line[i] > '0' && line[i] <= '9' {
            results.push(line[i]);
            *index = i as i32;
            return;
        }

        if line[i] == 'o' {
            if i+1 < line.len() && line[i+1] == 'n' {
                if i+2 < line.len() && line[i+2] == 'e' {
                    results.push('1');
                    *index = i as i32;
                    return;
                }
            }
        }

        if line[i] == 't' {
            if i+1 < line.len() && line[i+1] == 'w' {
                if i+2 < line.len() && line[i+2] == 'o' {
                    results.push('2');
                    *index = i as i32;
                    return;
                }
            }

            if i+1 < line.len() && line[i+1] == 'h' {
                if i+2 < line.len() && line[i+2] == 'r' {
                    if i+3 < line.len() && line[i+3] == 'e' {
                        if i+4 < line.len() && line[i+4] == 'e' {
                            results.push('3');
                            *index = i as i32;
                            return;
                        }
                    }
                }
            }
        }

        if i < line.len() && line[i] == 'f' {
            if i+1 < line.len() && line[i+1] == 'o' {
                if i+2 < line.len() && line[i+2] == 'u' {
                    if i+3 < line.len() && line[i+3] == 'r' {
                        results.push('4');
                        *index = i as i32;
                        return;
                    }
                }
            }

            if i+1 < line.len() && line[i+1] == 'i' {
                if i+2 < line.len() && line[i+2] == 'v' {
                    if i+3 < line.len() && line[i+3] == 'e' {
                        results.push('5');
                        *index = i as i32;
                        return;
                    }
                }
            }
        }

        if i < line.len() && line[i] == 's' {
            if i+1 < line.len() && line[i+1] == 'i' {
                if i+2 < line.len() && line[i+2] == 'x' {
                    results.push('6');
                    *index = i as i32;
                    return;
                }
            }

            if i+1 < line.len() && line[i+1] == 'e' {
                if i+2 < line.len() && line[i+2] == 'v' {
                    if i+3 < line.len() && line[i+3] == 'e' {
                        if i+4 < line.len() && line[i+4] == 'n' {
                            results.push('7');
                            *index = i as i32;
                            return;
                        }
                    }
                }
            }
        }

        if i < line.len() && line[i] == 'e' {
            if i+1 < line.len() && line[i+1] == 'i' {
                if i+2 < line.len() && line[i+2] == 'g' {
                    if i+3 < line.len() && line[i+3] == 'h' {
                        if i+4 < line.len() && line[i+4] == 't' {
                            results.push('8');
                            *index = i as i32;
                            return;
                        }
                    }
                }
            }
        }

        if i < line.len() && line[i] == 'n' {
            if i+1 < line.len() && line[i+1] == 'i' {
                if i+2 < line.len() && line[i+2] == 'n' {
                    if i+3 < line.len() && line[i+3] == 'e' {
                        results.push('9');
                        *index = i as i32;
                        return;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Game {
    id: String,
    guesses: Vec<Guess>
}

#[derive(Debug)]
struct Guess {
    pub count: i32,
    pub color: String
}

fn is_digit(c: char) -> bool {
    return c.is_digit(10);
}

fn is_space(c: char) -> bool {
    return c.is_whitespace();
}

fn parse_color(input: &str) -> IResult<&str, &str>{
    return preceded(multispace0, alpha1)(input);
}

fn parse_count(input: &str) -> IResult<&str, &str>{
    return preceded(multispace0, digit1)(input);
}

fn parse_guess(input: &str) -> IResult<&str, Guess> {

    let (input, count) = parse_count(input)?;
    let (input, _) = multispace0(input)?;
    let (input, color) = parse_color(input)?;

    return Ok((input, Guess { color: color.to_string(), count: count.parse::<i32>().unwrap()}));
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = multispace0(input)?;
    let (input, id) = delimited(tag_no_case("Game "), digit1, tag(":"))(input)?;
    let (input, moves) = separated_list1(alt((tag(";"), tag(","))), parse_guess)(input)?;

    return Ok((input, Game { id: id.to_string(), guesses: moves}));
}

fn day_two_one(red: i32, green: i32, blue: i32) -> i32 {

    let input1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    let input = "Game 1: 2 red, 2 green; 1 red, 1 green, 2 blue; 3 blue, 3 red, 3 green; 1 blue, 3 green, 7 red; 5 red, 3 green, 1 blue
    Game 2: 5 green, 4 red, 7 blue; 7 red, 4 green, 4 blue; 8 green, 11 blue, 4 red; 2 red, 18 blue, 3 green; 7 red, 15 blue
    Game 3: 2 green, 4 blue; 2 red, 2 green; 6 red, 1 green; 2 red, 1 green; 2 green; 5 blue, 5 red
    Game 4: 10 red, 7 green, 10 blue; 8 red, 2 green; 9 green, 6 red, 5 blue; 8 green, 2 blue, 4 red; 5 green, 9 blue; 10 red, 1 green, 9 blue
    Game 5: 10 blue, 7 green, 2 red; 2 blue, 4 red; 2 green, 9 blue, 8 red
    Game 6: 3 green, 8 red; 1 blue, 11 red, 2 green; 2 green, 15 red, 8 blue; 13 red, 6 blue, 3 green
    Game 7: 4 green, 10 red, 7 blue; 6 red, 9 blue, 9 green; 2 red, 1 blue, 6 green
    Game 8: 1 red, 3 blue, 2 green; 7 green, 2 blue; 10 green, 1 red, 2 blue; 1 red
    Game 9: 4 red, 3 green, 11 blue; 6 red, 4 green; 15 red, 7 blue, 7 green
    Game 10: 7 red, 1 blue, 5 green; 11 red, 7 green, 1 blue; 2 green, 4 blue, 13 red
    Game 11: 2 blue, 13 red, 12 green; 6 green, 5 red, 4 blue; 5 red, 11 green
    Game 12: 7 blue, 3 red, 11 green; 5 red, 1 blue, 8 green; 9 green, 7 blue, 8 red
    Game 13: 1 blue, 12 red; 9 red, 1 green, 1 blue; 8 red; 1 green, 4 red; 2 red
    Game 14: 6 blue, 5 green, 1 red; 12 blue, 4 red, 9 green; 7 green, 6 red; 8 blue, 10 green, 4 red; 8 green, 7 red
    Game 15: 15 blue, 10 red, 3 green; 9 green, 6 red, 11 blue; 3 green, 8 red, 5 blue; 12 green, 6 red, 16 blue; 11 red, 9 green, 15 blue
    Game 16: 12 green, 2 red, 7 blue; 9 red, 6 blue, 9 green; 7 green, 10 blue; 9 blue, 3 red, 9 green; 5 blue, 1 red
    Game 17: 4 green, 3 red, 11 blue; 8 green, 16 blue; 10 green, 12 blue, 2 red; 8 green, 2 red, 15 blue
    Game 18: 6 red, 8 green; 16 blue; 4 blue, 6 red; 16 blue, 10 green, 3 red; 12 blue, 15 green; 9 blue, 1 green, 4 red
    Game 19: 9 green, 9 red; 4 green, 13 red, 2 blue; 2 blue, 4 green, 3 red; 5 green, 3 blue, 3 red
    Game 20: 1 green, 6 red, 12 blue; 3 green, 8 red, 11 blue; 7 green, 5 red, 2 blue; 5 green, 14 blue, 5 red
    Game 21: 5 green, 1 blue, 13 red; 3 green, 13 red, 2 blue; 8 green, 12 red, 3 blue; 3 blue, 6 green, 9 red; 1 blue, 4 green, 13 red
    Game 22: 8 green, 14 red, 15 blue; 10 blue, 8 red, 14 green; 15 green, 15 blue, 6 red; 14 green, 10 blue, 7 red
    Game 23: 18 red, 9 green; 3 green, 1 blue, 17 red; 10 red, 16 green
    Game 24: 1 red, 2 blue, 4 green; 2 red, 5 blue, 3 green; 5 green, 5 blue; 8 blue, 1 red, 3 green; 2 green, 2 red, 6 blue; 2 green, 4 blue
    Game 25: 5 blue, 4 red, 1 green; 4 blue, 8 red, 1 green; 6 red, 5 blue; 8 red; 9 red, 3 blue; 1 green, 3 blue, 5 red
    Game 26: 20 blue, 4 red, 15 green; 10 red, 2 green, 12 blue; 7 blue, 15 green, 9 red; 1 red, 10 green, 5 blue; 14 green, 7 red, 15 blue
    Game 27: 17 red, 6 green; 6 green, 5 red, 3 blue; 4 green, 4 red, 5 blue; 3 green, 3 blue, 16 red; 4 blue, 5 green, 15 red
    Game 28: 5 blue, 6 green, 1 red; 13 blue; 1 red, 9 blue, 10 green
    Game 29: 1 red, 10 blue; 9 green, 6 blue, 3 red; 17 green, 1 red, 9 blue; 7 blue, 1 red; 1 red, 15 blue, 9 green; 7 green, 1 red, 4 blue
    Game 30: 3 red, 11 blue, 2 green; 11 green, 8 blue, 8 red; 1 red, 3 green; 19 green, 11 blue
    Game 31: 19 green, 6 red; 4 green, 10 red; 12 green, 1 blue
    Game 32: 4 green, 3 blue, 10 red; 4 red, 6 blue, 3 green; 10 red, 5 blue
    Game 33: 2 blue, 5 green, 5 red; 4 blue, 2 green, 4 red; 13 red, 2 green; 7 blue, 4 green, 2 red; 19 blue, 5 green, 11 red; 4 green, 18 blue, 1 red
    Game 34: 6 blue, 9 red, 7 green; 7 green, 6 red, 12 blue; 3 red, 6 green, 16 blue; 3 green, 15 blue, 13 red; 2 green, 16 blue, 3 red
    Game 35: 4 green; 3 green, 4 red, 1 blue; 6 red, 12 green, 2 blue
    Game 36: 1 blue, 8 red, 3 green; 10 red, 5 green; 1 green, 8 red; 4 green, 1 blue, 11 red
    Game 37: 2 red, 4 blue, 5 green; 2 green, 1 blue, 3 red; 8 green, 3 red, 4 blue; 1 blue, 8 green, 2 red
    Game 38: 11 green, 4 blue; 2 blue, 11 green, 1 red; 12 green, 7 blue, 1 red; 7 blue, 10 green, 1 red; 13 green, 2 red; 1 red, 7 blue, 2 green
    Game 39: 7 green, 1 red, 15 blue; 8 red, 7 blue; 15 red, 5 green, 6 blue
    Game 40: 2 green, 12 blue, 15 red; 2 green, 6 red; 5 green, 9 red; 9 blue, 12 red; 4 green, 12 red, 12 blue; 12 red, 8 blue, 2 green
    Game 41: 9 blue, 6 red, 3 green; 6 red, 2 green, 9 blue; 1 blue, 11 red
    Game 42: 4 red, 3 blue, 13 green; 5 blue, 11 red, 15 green; 3 red, 12 green; 2 red, 6 blue, 3 green
    Game 43: 2 green, 7 red; 11 red, 18 green, 1 blue; 13 red, 12 green, 1 blue; 15 red; 5 red, 19 green; 15 green, 5 red
    Game 44: 2 red, 5 green, 7 blue; 5 green, 8 blue; 8 red, 8 green; 1 green, 1 red, 6 blue; 1 blue, 1 red
    Game 45: 3 red, 3 green, 7 blue; 12 red, 17 blue; 7 green, 8 red, 14 blue; 9 green, 10 red, 13 blue; 15 green, 16 blue, 4 red
    Game 46: 2 blue, 5 green; 4 red, 7 green; 15 red, 7 green
    Game 47: 5 red, 9 green, 4 blue; 1 red, 9 green, 11 blue; 8 green, 1 red; 4 red, 4 blue, 3 green; 10 blue, 14 green
    Game 48: 1 red, 14 blue, 11 green; 3 blue, 8 green; 5 green, 5 blue; 5 blue, 1 red, 8 green; 10 green, 2 red, 6 blue
    Game 49: 11 blue, 5 red, 3 green; 7 blue, 12 red, 4 green; 9 green, 6 red; 4 green, 3 blue, 10 red
    Game 50: 3 red, 8 blue, 13 green; 13 blue, 13 green; 3 green, 10 blue, 1 red; 12 green, 15 blue; 12 blue, 3 red, 8 green; 5 blue, 5 red, 4 green
    Game 51: 3 green, 1 blue; 1 red; 1 green, 7 blue
    Game 52: 3 red, 4 blue; 4 blue, 1 green, 2 red; 1 green, 3 red; 5 red, 1 green; 1 blue, 1 red, 1 green
    Game 53: 5 red, 17 green, 4 blue; 15 red, 14 blue, 1 green; 9 blue, 5 green; 3 blue, 5 red, 9 green; 1 green, 15 blue, 10 red; 16 green, 10 blue
    Game 54: 4 blue, 7 red, 1 green; 7 green, 8 red, 6 blue; 14 green, 1 blue, 5 red
    Game 55: 4 blue, 4 green, 1 red; 1 green; 3 red
    Game 56: 3 green, 1 red, 7 blue; 1 blue, 2 red, 3 green; 2 green, 9 red; 14 red, 8 blue, 1 green; 5 red, 13 blue; 6 red, 3 blue
    Game 57: 15 green, 5 red, 5 blue; 13 green, 13 blue, 12 red; 18 green, 5 blue, 8 red; 7 green, 7 blue, 13 red
    Game 58: 4 red, 2 blue, 6 green; 4 red, 3 green, 14 blue; 9 green, 3 red; 3 red, 5 blue, 11 green
    Game 59: 2 red, 6 green, 1 blue; 5 blue, 1 green, 4 red; 2 red, 7 green, 6 blue; 3 green, 6 blue; 1 blue, 6 green
    Game 60: 4 red, 9 green, 3 blue; 2 blue, 8 green, 6 red; 2 red, 8 green, 3 blue; 8 green, 2 red, 2 blue
    Game 61: 12 red, 4 blue, 3 green; 1 blue, 2 green; 2 red, 2 green, 3 blue
    Game 62: 4 red, 6 green, 14 blue; 12 green, 2 red, 4 blue; 5 blue, 5 red, 7 green
    Game 63: 1 green, 5 red; 5 red, 1 blue, 1 green; 1 blue
    Game 64: 6 red, 9 green, 4 blue; 8 red, 13 green; 3 blue, 8 red, 11 green; 5 red, 1 blue, 2 green; 3 blue, 7 red, 1 green
    Game 65: 15 green, 10 red, 1 blue; 1 blue, 2 red, 4 green; 10 blue, 4 green
    Game 66: 13 blue, 6 red, 2 green; 13 green; 10 blue, 8 green; 7 red, 10 blue, 11 green; 10 green, 1 red, 8 blue
    Game 67: 5 blue, 4 green, 1 red; 2 green, 4 blue, 1 red; 7 green, 2 blue, 1 red; 1 blue, 1 green
    Game 68: 2 green, 12 blue, 3 red; 5 red, 14 blue, 2 green; 6 red, 14 blue; 10 blue, 6 red, 2 green
    Game 69: 7 blue, 1 red, 12 green; 10 blue, 11 green, 6 red; 4 red, 10 green, 7 blue
    Game 70: 4 blue; 6 red, 2 green, 11 blue; 4 green, 3 blue, 2 red; 14 blue, 2 red, 4 green
    Game 71: 5 red, 17 blue; 9 blue, 11 red, 1 green; 19 blue, 6 red; 4 red, 2 blue
    Game 72: 2 green, 5 red, 1 blue; 4 green, 4 red; 4 green, 2 red; 2 blue, 2 green; 1 blue, 1 green, 5 red
    Game 73: 4 red, 3 blue, 1 green; 10 red, 2 blue, 3 green; 14 red, 1 green, 2 blue; 1 blue; 3 green, 9 red, 6 blue; 11 red, 7 blue, 2 green
    Game 74: 1 red, 5 blue, 10 green; 2 red, 9 blue, 9 green; 8 green, 2 red, 4 blue; 10 blue, 9 green; 12 green, 3 red, 5 blue
    Game 75: 3 red, 13 blue, 6 green; 3 green, 1 red; 9 green, 1 blue, 5 red; 5 green, 13 red, 4 blue; 13 green, 2 blue, 10 red; 9 green, 3 red, 10 blue
    Game 76: 14 green, 2 red, 16 blue; 2 blue, 1 red, 7 green; 14 green, 9 blue, 8 red
    Game 77: 1 green, 1 blue; 1 green; 3 red, 3 blue, 1 green; 3 green, 3 red; 1 red, 2 blue
    Game 78: 4 red, 13 green; 17 green, 1 blue, 2 red; 8 red, 14 green
    Game 79: 4 green, 10 red, 6 blue; 5 blue, 3 red, 7 green; 6 blue, 2 red, 4 green; 2 blue, 8 red
    Game 80: 19 green, 5 red; 5 green, 9 blue; 3 red, 18 blue, 10 green; 2 red, 15 green, 7 blue; 4 red, 14 green, 15 blue
    Game 81: 10 red, 2 blue, 1 green; 18 red, 3 blue; 6 red, 12 blue; 1 green, 3 red, 3 blue
    Game 82: 8 green, 1 blue; 2 blue, 4 red; 7 green, 1 red, 4 blue; 2 green, 3 red, 2 blue; 3 red; 4 red, 8 green, 1 blue
    Game 83: 3 green, 1 blue; 1 red, 2 blue, 14 green; 8 red, 17 green
    Game 84: 7 green, 4 blue, 4 red; 11 green, 17 red, 11 blue; 9 green, 5 blue, 14 red; 9 green, 10 blue, 5 red
    Game 85: 1 red, 1 green; 1 blue, 8 red, 1 green; 8 green, 1 red; 8 green, 2 red, 1 blue
    Game 86: 1 red, 5 blue, 1 green; 1 green, 7 red; 8 red; 3 blue, 2 red
    Game 87: 7 red, 8 blue, 1 green; 8 red, 6 green; 6 red, 8 green, 10 blue
    Game 88: 5 red, 4 green, 5 blue; 1 blue, 2 green; 6 green, 10 blue, 4 red; 1 red, 8 green, 1 blue
    Game 89: 3 green, 7 blue, 11 red; 1 blue, 5 green, 18 red; 1 blue, 3 green, 13 red; 7 blue, 9 green, 3 red; 1 green, 8 blue, 19 red; 4 blue, 15 red, 1 green
    Game 90: 3 blue, 3 red, 4 green; 14 red, 6 green, 4 blue; 1 blue, 9 red; 6 red, 1 green; 5 green, 8 red, 2 blue; 3 blue, 4 red, 3 green
    Game 91: 1 red, 1 blue, 16 green; 8 red, 5 green; 1 blue, 2 red, 10 green; 3 red, 15 green, 1 blue
    Game 92: 10 green, 12 blue; 6 red, 6 blue; 5 red, 12 blue; 6 red, 9 green, 2 blue; 10 blue, 3 red, 1 green; 1 red, 19 blue, 11 green
    Game 93: 4 green; 5 green, 2 blue, 3 red; 1 blue, 3 red, 6 green; 2 blue, 2 red, 7 green
    Game 94: 4 blue, 2 red; 6 green, 6 blue, 4 red; 8 green, 1 blue, 3 red
    Game 95: 6 green, 4 blue, 15 red; 13 red, 7 blue, 3 green; 14 red, 5 blue, 6 green; 5 blue, 7 red, 2 green
    Game 96: 1 red, 1 blue, 11 green; 6 blue, 2 red, 14 green; 3 green, 2 red; 9 blue, 10 green
    Game 97: 10 green; 2 red, 4 green, 1 blue; 2 green, 1 red; 2 red, 1 blue, 10 green; 1 green
    Game 98: 1 green, 5 blue; 2 green, 7 blue, 4 red; 2 red, 1 green, 9 blue; 4 blue, 4 red
    Game 99: 3 green, 1 red, 3 blue; 12 green, 12 blue, 4 red; 12 blue, 2 red, 10 green; 4 blue, 2 red, 4 green
    Game 100: 1 red, 5 blue, 2 green; 3 red, 1 blue; 1 green, 1 blue, 1 red";

    let lines = input.split("\n");

    let mut all_maps: Vec<(i32, HashMap<String, i32>)> = vec![];

    for line in lines {
        match parse_game(line) {
            Ok((_, game)) => {
                let res: Vec<(String, i32)> = game.guesses.iter().map(|game| (game.color.clone(), game.count)).collect();
                let mut res_map: HashMap<String, i32> = HashMap::new();

                for r in res {
                    match res_map.get(&r.0) {
                        Some(v) => {
                            res_map.insert(r.0, r.1.max(*v));
                        }
                        None => {
                            res_map.insert(r.0, r.1);
                        }
                    }
                }

                all_maps.push((game.id.trim().parse::<i32>().unwrap(), res_map));
            }

            Err(error) => {
                println!("{}", error);
            }
        }
    }

    let mut total = 0;

    for m in all_maps.clone() {
        println!("{:?} {}", m, all_maps.len());

        match m.1.get("red") {
            None => {
                if red == 0 {
                    continue;
                }
            }
            Some(v) => {
                println!("sent: {} bag: {}", red, v);
                if red < *v {
                    continue;
                }
            }
        }

        match m.1.get("blue") {
            None => {
                if blue == 0 {
                    continue;
                }
            }
            Some(v) => {
                println!("sent: {} bag: {}", blue, v);
                if blue < *v {
                    continue;
                }
            }
        }

        match m.1.get("green") {
            None => {
                if green == 0 {
                    continue;
                }
            }
            Some(v) => {
                println!("sent: {} bag: {}", green, v);
                if green < *v {
                    continue;
                }
            }
        }

        total += m.0;

    }

    println!("{:?}", all_maps);
    println!("{}", total);
    return total;
}


fn day_two_two() -> i32 {

    let input = "Game 1: 2 red, 2 green; 1 red, 1 green, 2 blue; 3 blue, 3 red, 3 green; 1 blue, 3 green, 7 red; 5 red, 3 green, 1 blue
    Game 2: 5 green, 4 red, 7 blue; 7 red, 4 green, 4 blue; 8 green, 11 blue, 4 red; 2 red, 18 blue, 3 green; 7 red, 15 blue
    Game 3: 2 green, 4 blue; 2 red, 2 green; 6 red, 1 green; 2 red, 1 green; 2 green; 5 blue, 5 red
    Game 4: 10 red, 7 green, 10 blue; 8 red, 2 green; 9 green, 6 red, 5 blue; 8 green, 2 blue, 4 red; 5 green, 9 blue; 10 red, 1 green, 9 blue
    Game 5: 10 blue, 7 green, 2 red; 2 blue, 4 red; 2 green, 9 blue, 8 red
    Game 6: 3 green, 8 red; 1 blue, 11 red, 2 green; 2 green, 15 red, 8 blue; 13 red, 6 blue, 3 green
    Game 7: 4 green, 10 red, 7 blue; 6 red, 9 blue, 9 green; 2 red, 1 blue, 6 green
    Game 8: 1 red, 3 blue, 2 green; 7 green, 2 blue; 10 green, 1 red, 2 blue; 1 red
    Game 9: 4 red, 3 green, 11 blue; 6 red, 4 green; 15 red, 7 blue, 7 green
    Game 10: 7 red, 1 blue, 5 green; 11 red, 7 green, 1 blue; 2 green, 4 blue, 13 red
    Game 11: 2 blue, 13 red, 12 green; 6 green, 5 red, 4 blue; 5 red, 11 green
    Game 12: 7 blue, 3 red, 11 green; 5 red, 1 blue, 8 green; 9 green, 7 blue, 8 red
    Game 13: 1 blue, 12 red; 9 red, 1 green, 1 blue; 8 red; 1 green, 4 red; 2 red
    Game 14: 6 blue, 5 green, 1 red; 12 blue, 4 red, 9 green; 7 green, 6 red; 8 blue, 10 green, 4 red; 8 green, 7 red
    Game 15: 15 blue, 10 red, 3 green; 9 green, 6 red, 11 blue; 3 green, 8 red, 5 blue; 12 green, 6 red, 16 blue; 11 red, 9 green, 15 blue
    Game 16: 12 green, 2 red, 7 blue; 9 red, 6 blue, 9 green; 7 green, 10 blue; 9 blue, 3 red, 9 green; 5 blue, 1 red
    Game 17: 4 green, 3 red, 11 blue; 8 green, 16 blue; 10 green, 12 blue, 2 red; 8 green, 2 red, 15 blue
    Game 18: 6 red, 8 green; 16 blue; 4 blue, 6 red; 16 blue, 10 green, 3 red; 12 blue, 15 green; 9 blue, 1 green, 4 red
    Game 19: 9 green, 9 red; 4 green, 13 red, 2 blue; 2 blue, 4 green, 3 red; 5 green, 3 blue, 3 red
    Game 20: 1 green, 6 red, 12 blue; 3 green, 8 red, 11 blue; 7 green, 5 red, 2 blue; 5 green, 14 blue, 5 red
    Game 21: 5 green, 1 blue, 13 red; 3 green, 13 red, 2 blue; 8 green, 12 red, 3 blue; 3 blue, 6 green, 9 red; 1 blue, 4 green, 13 red
    Game 22: 8 green, 14 red, 15 blue; 10 blue, 8 red, 14 green; 15 green, 15 blue, 6 red; 14 green, 10 blue, 7 red
    Game 23: 18 red, 9 green; 3 green, 1 blue, 17 red; 10 red, 16 green
    Game 24: 1 red, 2 blue, 4 green; 2 red, 5 blue, 3 green; 5 green, 5 blue; 8 blue, 1 red, 3 green; 2 green, 2 red, 6 blue; 2 green, 4 blue
    Game 25: 5 blue, 4 red, 1 green; 4 blue, 8 red, 1 green; 6 red, 5 blue; 8 red; 9 red, 3 blue; 1 green, 3 blue, 5 red
    Game 26: 20 blue, 4 red, 15 green; 10 red, 2 green, 12 blue; 7 blue, 15 green, 9 red; 1 red, 10 green, 5 blue; 14 green, 7 red, 15 blue
    Game 27: 17 red, 6 green; 6 green, 5 red, 3 blue; 4 green, 4 red, 5 blue; 3 green, 3 blue, 16 red; 4 blue, 5 green, 15 red
    Game 28: 5 blue, 6 green, 1 red; 13 blue; 1 red, 9 blue, 10 green
    Game 29: 1 red, 10 blue; 9 green, 6 blue, 3 red; 17 green, 1 red, 9 blue; 7 blue, 1 red; 1 red, 15 blue, 9 green; 7 green, 1 red, 4 blue
    Game 30: 3 red, 11 blue, 2 green; 11 green, 8 blue, 8 red; 1 red, 3 green; 19 green, 11 blue
    Game 31: 19 green, 6 red; 4 green, 10 red; 12 green, 1 blue
    Game 32: 4 green, 3 blue, 10 red; 4 red, 6 blue, 3 green; 10 red, 5 blue
    Game 33: 2 blue, 5 green, 5 red; 4 blue, 2 green, 4 red; 13 red, 2 green; 7 blue, 4 green, 2 red; 19 blue, 5 green, 11 red; 4 green, 18 blue, 1 red
    Game 34: 6 blue, 9 red, 7 green; 7 green, 6 red, 12 blue; 3 red, 6 green, 16 blue; 3 green, 15 blue, 13 red; 2 green, 16 blue, 3 red
    Game 35: 4 green; 3 green, 4 red, 1 blue; 6 red, 12 green, 2 blue
    Game 36: 1 blue, 8 red, 3 green; 10 red, 5 green; 1 green, 8 red; 4 green, 1 blue, 11 red
    Game 37: 2 red, 4 blue, 5 green; 2 green, 1 blue, 3 red; 8 green, 3 red, 4 blue; 1 blue, 8 green, 2 red
    Game 38: 11 green, 4 blue; 2 blue, 11 green, 1 red; 12 green, 7 blue, 1 red; 7 blue, 10 green, 1 red; 13 green, 2 red; 1 red, 7 blue, 2 green
    Game 39: 7 green, 1 red, 15 blue; 8 red, 7 blue; 15 red, 5 green, 6 blue
    Game 40: 2 green, 12 blue, 15 red; 2 green, 6 red; 5 green, 9 red; 9 blue, 12 red; 4 green, 12 red, 12 blue; 12 red, 8 blue, 2 green
    Game 41: 9 blue, 6 red, 3 green; 6 red, 2 green, 9 blue; 1 blue, 11 red
    Game 42: 4 red, 3 blue, 13 green; 5 blue, 11 red, 15 green; 3 red, 12 green; 2 red, 6 blue, 3 green
    Game 43: 2 green, 7 red; 11 red, 18 green, 1 blue; 13 red, 12 green, 1 blue; 15 red; 5 red, 19 green; 15 green, 5 red
    Game 44: 2 red, 5 green, 7 blue; 5 green, 8 blue; 8 red, 8 green; 1 green, 1 red, 6 blue; 1 blue, 1 red
    Game 45: 3 red, 3 green, 7 blue; 12 red, 17 blue; 7 green, 8 red, 14 blue; 9 green, 10 red, 13 blue; 15 green, 16 blue, 4 red
    Game 46: 2 blue, 5 green; 4 red, 7 green; 15 red, 7 green
    Game 47: 5 red, 9 green, 4 blue; 1 red, 9 green, 11 blue; 8 green, 1 red; 4 red, 4 blue, 3 green; 10 blue, 14 green
    Game 48: 1 red, 14 blue, 11 green; 3 blue, 8 green; 5 green, 5 blue; 5 blue, 1 red, 8 green; 10 green, 2 red, 6 blue
    Game 49: 11 blue, 5 red, 3 green; 7 blue, 12 red, 4 green; 9 green, 6 red; 4 green, 3 blue, 10 red
    Game 50: 3 red, 8 blue, 13 green; 13 blue, 13 green; 3 green, 10 blue, 1 red; 12 green, 15 blue; 12 blue, 3 red, 8 green; 5 blue, 5 red, 4 green
    Game 51: 3 green, 1 blue; 1 red; 1 green, 7 blue
    Game 52: 3 red, 4 blue; 4 blue, 1 green, 2 red; 1 green, 3 red; 5 red, 1 green; 1 blue, 1 red, 1 green
    Game 53: 5 red, 17 green, 4 blue; 15 red, 14 blue, 1 green; 9 blue, 5 green; 3 blue, 5 red, 9 green; 1 green, 15 blue, 10 red; 16 green, 10 blue
    Game 54: 4 blue, 7 red, 1 green; 7 green, 8 red, 6 blue; 14 green, 1 blue, 5 red
    Game 55: 4 blue, 4 green, 1 red; 1 green; 3 red
    Game 56: 3 green, 1 red, 7 blue; 1 blue, 2 red, 3 green; 2 green, 9 red; 14 red, 8 blue, 1 green; 5 red, 13 blue; 6 red, 3 blue
    Game 57: 15 green, 5 red, 5 blue; 13 green, 13 blue, 12 red; 18 green, 5 blue, 8 red; 7 green, 7 blue, 13 red
    Game 58: 4 red, 2 blue, 6 green; 4 red, 3 green, 14 blue; 9 green, 3 red; 3 red, 5 blue, 11 green
    Game 59: 2 red, 6 green, 1 blue; 5 blue, 1 green, 4 red; 2 red, 7 green, 6 blue; 3 green, 6 blue; 1 blue, 6 green
    Game 60: 4 red, 9 green, 3 blue; 2 blue, 8 green, 6 red; 2 red, 8 green, 3 blue; 8 green, 2 red, 2 blue
    Game 61: 12 red, 4 blue, 3 green; 1 blue, 2 green; 2 red, 2 green, 3 blue
    Game 62: 4 red, 6 green, 14 blue; 12 green, 2 red, 4 blue; 5 blue, 5 red, 7 green
    Game 63: 1 green, 5 red; 5 red, 1 blue, 1 green; 1 blue
    Game 64: 6 red, 9 green, 4 blue; 8 red, 13 green; 3 blue, 8 red, 11 green; 5 red, 1 blue, 2 green; 3 blue, 7 red, 1 green
    Game 65: 15 green, 10 red, 1 blue; 1 blue, 2 red, 4 green; 10 blue, 4 green
    Game 66: 13 blue, 6 red, 2 green; 13 green; 10 blue, 8 green; 7 red, 10 blue, 11 green; 10 green, 1 red, 8 blue
    Game 67: 5 blue, 4 green, 1 red; 2 green, 4 blue, 1 red; 7 green, 2 blue, 1 red; 1 blue, 1 green
    Game 68: 2 green, 12 blue, 3 red; 5 red, 14 blue, 2 green; 6 red, 14 blue; 10 blue, 6 red, 2 green
    Game 69: 7 blue, 1 red, 12 green; 10 blue, 11 green, 6 red; 4 red, 10 green, 7 blue
    Game 70: 4 blue; 6 red, 2 green, 11 blue; 4 green, 3 blue, 2 red; 14 blue, 2 red, 4 green
    Game 71: 5 red, 17 blue; 9 blue, 11 red, 1 green; 19 blue, 6 red; 4 red, 2 blue
    Game 72: 2 green, 5 red, 1 blue; 4 green, 4 red; 4 green, 2 red; 2 blue, 2 green; 1 blue, 1 green, 5 red
    Game 73: 4 red, 3 blue, 1 green; 10 red, 2 blue, 3 green; 14 red, 1 green, 2 blue; 1 blue; 3 green, 9 red, 6 blue; 11 red, 7 blue, 2 green
    Game 74: 1 red, 5 blue, 10 green; 2 red, 9 blue, 9 green; 8 green, 2 red, 4 blue; 10 blue, 9 green; 12 green, 3 red, 5 blue
    Game 75: 3 red, 13 blue, 6 green; 3 green, 1 red; 9 green, 1 blue, 5 red; 5 green, 13 red, 4 blue; 13 green, 2 blue, 10 red; 9 green, 3 red, 10 blue
    Game 76: 14 green, 2 red, 16 blue; 2 blue, 1 red, 7 green; 14 green, 9 blue, 8 red
    Game 77: 1 green, 1 blue; 1 green; 3 red, 3 blue, 1 green; 3 green, 3 red; 1 red, 2 blue
    Game 78: 4 red, 13 green; 17 green, 1 blue, 2 red; 8 red, 14 green
    Game 79: 4 green, 10 red, 6 blue; 5 blue, 3 red, 7 green; 6 blue, 2 red, 4 green; 2 blue, 8 red
    Game 80: 19 green, 5 red; 5 green, 9 blue; 3 red, 18 blue, 10 green; 2 red, 15 green, 7 blue; 4 red, 14 green, 15 blue
    Game 81: 10 red, 2 blue, 1 green; 18 red, 3 blue; 6 red, 12 blue; 1 green, 3 red, 3 blue
    Game 82: 8 green, 1 blue; 2 blue, 4 red; 7 green, 1 red, 4 blue; 2 green, 3 red, 2 blue; 3 red; 4 red, 8 green, 1 blue
    Game 83: 3 green, 1 blue; 1 red, 2 blue, 14 green; 8 red, 17 green
    Game 84: 7 green, 4 blue, 4 red; 11 green, 17 red, 11 blue; 9 green, 5 blue, 14 red; 9 green, 10 blue, 5 red
    Game 85: 1 red, 1 green; 1 blue, 8 red, 1 green; 8 green, 1 red; 8 green, 2 red, 1 blue
    Game 86: 1 red, 5 blue, 1 green; 1 green, 7 red; 8 red; 3 blue, 2 red
    Game 87: 7 red, 8 blue, 1 green; 8 red, 6 green; 6 red, 8 green, 10 blue
    Game 88: 5 red, 4 green, 5 blue; 1 blue, 2 green; 6 green, 10 blue, 4 red; 1 red, 8 green, 1 blue
    Game 89: 3 green, 7 blue, 11 red; 1 blue, 5 green, 18 red; 1 blue, 3 green, 13 red; 7 blue, 9 green, 3 red; 1 green, 8 blue, 19 red; 4 blue, 15 red, 1 green
    Game 90: 3 blue, 3 red, 4 green; 14 red, 6 green, 4 blue; 1 blue, 9 red; 6 red, 1 green; 5 green, 8 red, 2 blue; 3 blue, 4 red, 3 green
    Game 91: 1 red, 1 blue, 16 green; 8 red, 5 green; 1 blue, 2 red, 10 green; 3 red, 15 green, 1 blue
    Game 92: 10 green, 12 blue; 6 red, 6 blue; 5 red, 12 blue; 6 red, 9 green, 2 blue; 10 blue, 3 red, 1 green; 1 red, 19 blue, 11 green
    Game 93: 4 green; 5 green, 2 blue, 3 red; 1 blue, 3 red, 6 green; 2 blue, 2 red, 7 green
    Game 94: 4 blue, 2 red; 6 green, 6 blue, 4 red; 8 green, 1 blue, 3 red
    Game 95: 6 green, 4 blue, 15 red; 13 red, 7 blue, 3 green; 14 red, 5 blue, 6 green; 5 blue, 7 red, 2 green
    Game 96: 1 red, 1 blue, 11 green; 6 blue, 2 red, 14 green; 3 green, 2 red; 9 blue, 10 green
    Game 97: 10 green; 2 red, 4 green, 1 blue; 2 green, 1 red; 2 red, 1 blue, 10 green; 1 green
    Game 98: 1 green, 5 blue; 2 green, 7 blue, 4 red; 2 red, 1 green, 9 blue; 4 blue, 4 red
    Game 99: 3 green, 1 red, 3 blue; 12 green, 12 blue, 4 red; 12 blue, 2 red, 10 green; 4 blue, 2 red, 4 green
    Game 100: 1 red, 5 blue, 2 green; 3 red, 1 blue; 1 green, 1 blue, 1 red";

    let lines = input.split("\n");

    let mut all_maps: Vec<(i32, HashMap<String, i32>)> = vec![];

    for line in lines {
        match parse_game(line) {
            Ok((_, game)) => {
                let res: Vec<(String, i32)> = game.guesses.iter().map(|game| (game.color.clone(), game.count)).collect();
                let mut res_map: HashMap<String, i32> = HashMap::new();

                for r in res {
                    match res_map.get(&r.0) {
                        Some(v) => {
                            res_map.insert(r.0, r.1.max(*v));
                        }
                        None => {
                            res_map.insert(r.0, r.1);
                        }
                    }
                }

                all_maps.push((game.id.trim().parse::<i32>().unwrap(), res_map));
            }

            Err(error) => {
                println!("{}", error);
            }
        }
    }

    let mut total = 0;

    for m in all_maps.clone() {
        println!("{:?} {}", m, all_maps.len());

        total += m.1.get("red").unwrap_or(&0) * m.1.get("green").unwrap_or(&0) * m.1.get("blue").unwrap_or(&0);
    }

    println!("{}", total);
    return total;
}

fn check_if_symbol(c: char) -> (bool, bool) {
    if c == '*' {
        return (true, true);
    }

    return (!c.is_digit(10) && c != '.', false);
}

fn check_adjacent(m: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    
    let height = m.len();
    let width = m[0].len();
    let mut adjacent = false;
    let mut is_gear = false;
    let mut gear_location = (0,0);
    if y > 0 && x > 0 {
        if check_if_symbol(m[y - 1][x - 1]).0 {
//            println!("top left");
            adjacent = true;
        }
    }

    if y < height - 1 && x < width - 1 {
        if check_if_symbol(m[y + 1][x + 1]).0 {
//            println!("bottom right {} {} {} {}", y, x, m[y][x], m[y+1][x+1]);
            adjacent = true;
        }
    }

    if y > 0 && x < width - 1 {
        if check_if_symbol(m[y - 1][x + 1]).0 {
//            println!("top right");
            adjacent = true;
        }
    }

    if y < height - 1 && x > 0 {
        if check_if_symbol(m[y + 1][x - 1]).0 {
//            println!("bottom left");
            adjacent = true;
        }
    }

    if y > 0 {
        if check_if_symbol(m[y - 1][x]).0 {
//            println!("top");
            adjacent = true;
        }
    }

    if y < height - 1 {
        if check_if_symbol(m[y + 1][x]).0 {
//           println!("bottom");
            adjacent = true;
        }
    }

    if x > 0 {
        if check_if_symbol(m[y][x - 1]).0 {
//            println!("left");
            adjacent = true;
        }
    }

    if x < width - 1 {
        if check_if_symbol(m[y][x + 1]).0 {
//            println!("right");
            adjacent = true;
        }
    }

    return adjacent;
}

fn check_adjacent_gears(m: &Vec<Vec<char>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
    
    let height = m.len();
    let width = m[0].len();
    let mut gears: HashSet<(usize, usize)> = HashSet::new();

    if y > 0 && x > 0 {
        let symbol = check_if_symbol(m[y - 1][x - 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y - 1, x - 1));
            }
        }
    }

    if y < height - 1 && x < width - 1 {

        let symbol = check_if_symbol(m[y + 1][x + 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y + 1, x + 1));
            }
        }
    }

    if y > 0 && x < width - 1 {
        let symbol = check_if_symbol(m[y - 1][x + 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y - 1, x + 1));
            }
        }
    }

    if y < height - 1 && x > 0 {
        let symbol = check_if_symbol(m[y + 1][x - 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y + 1, x - 1));
            }
        }
    }

    if y > 0 {
        let symbol = check_if_symbol(m[y - 1][x]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y - 1, x));
            }
        }
    }

    if y < height - 1 {
        let symbol = check_if_symbol(m[y + 1][x]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y + 1, x));
            }
        }
    }

    if x > 0 {
        let symbol = check_if_symbol(m[y][x - 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y, x - 1));
            }
        }
    }

    if x < width - 1 {
        let symbol = check_if_symbol(m[y][x + 1]);
        if symbol.0 {
//            println!("top left");

            if symbol.1 {
                gears.insert((y, x + 1));
            }
        }
    }

    return gears;
}

fn day_three_two() {

    let input2 = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    let input: &str = 
    "...................305.124................................432..............................................576..313.....514.................
    .............113...-......&....................&...819...........654..../..........................&901................*....869.257.........
    ...377..&783../.................................9...........855*......940..463................-.........................844.*....@......679.
    ......*...........197.261.....817..336.759............&742......548.......&........748......844.............#.......&........254...169..*...
    .......36....368.*...............*....*.........*..88......%866.......135.........*..................515.682.....114...%...........*.....768
    ...233......*....909..698.........427..........127.*...................*...........450.........482..../.................312.....621.........
    .../.......882...............776...................555......180.........971....217.......857.../........212....674.917......736.....441*760.
    ............................*.....@....907....%940.....%.......+.#..........45*...........$............#..........*...............=.........
    ..................370.225.425..211...................932..........381..267.............45.....=....549....238........367..&.....488.........
    .468.............*.......................761...677.......&.................929..907.....%....79...*.......*....324..*....515................
    ....-.#103.504...388...........$..........*.....@........766.%................*....*623.........532..881...573....*..840..............497...
    ...........*.........305....412..687.......971................676....@640....657........709......................807............98.....+....
    ....279...502...62...*.............*..............................................838%.....$.........585..........................*.........
    ....=...=......*....209.825.482.993..........972............620*664..-759............................*.......184.........952....160.....156.
    .......814.....931......*...*............551..*....711...65.................224*115...851.772..104..505.....=........796*...................
    ....................723.103..275........*......564..*.........830........................*...../..............................232...........
    .............$......&....................951........703......=............160+.....138......*........257*809.............$298........284....
    .....684....464.333..................827.....*211..................................#.....855.86.....................$913............*.......
    .......*...........*..*......917.163....*.409......294.......................#..39.......................511....358.......$671.......237....
    .......663.......311..163....*...%...158........*....-.442.........958....108.....#.....941*837.........*..........*........................
    ............865=..........409....................128...*.......283....................................680.........730.935@........768.......
    ....../................=.............54.....857.........418.....*..911.......610..643............381...............................@........
    .......925............984...........-...800*.................194.../.........*.......*.........../.....................................@....
    .................................@...............................&......615...16....695.....-..........................*750.............855.
    ....611..136...................108.540.........362..711......*....420.....................190.......................213.............86......
    204*....@.........288..825..+........&...........*....*......733.........................................7..387.......................*.....
    ...........795..../.......+.505.834.....228....192...619..........886......467.....942.............................%838.........512...340...
    .......957.........................*...*..................962........................*..............211...$..............624................
    .......*....................349....144......./786..350.......*.......................541..21.......$.....284..190*......*............518....
    ....484...@....17..........*..../........309..........*....530...118..........208..............253................489.828...........*.......
    ..........309...........715.....531........+.516#...645.........$........369....*.......*185...@............=.#...........@............&....
    491*192..............................................................233.........711.948.........572.....869..289......932...........149....
    ........274.....511.....425..........421.288*965...652=.919........./....*.786...........676.....*..................+.........=.............
    .........*.........@......$...*.........................*........*....791...+..113......*....527..897..499+.904...166........580.........389
    .......306.....................353.497..929/.......=....553......495.................884.....&........................794........774........
    ...................../...........................582....................................................489..........*...............&......
    .605......537.....247.....................479................74................227*995.761.....@...........+.......958...87.....547.795.....
    ...*.....*....................289+.....*.....%........967..............*...............*........611.....28....367.................*.........
    ..690....916.497....................894.440......547$.............596.492......23...............................*...............237.221..786
    .........................681.....=..........580+.................+...............+.......696............@..542..56..........................
    ...936..639*947.............+...48.*709..................307.257...-248.697.553.............+.........146.....#.......*241..................
    ...*................568....................=378....41.......*............*...%..11......*..........................982.............#........
    ..323..704.+724..*....*..195*....812..879...........*.....*....508@.....852....$.....664.......317....*......332..............326.158.......
    .......*........610..767.....936...#...&..744.290..839.382.543....................................&.827...../.............323*.........884..
    ...........962...............................*...................46....502.265.150...................................639...............*....
    955*818.......*...@...124*512..256...................+....&..............*....*.......170....996...#........................786.......4.....
    ..............284.697...........*......172...........398.435.......-.....380..........+..........698....428...361................206........
    .822...#..............936$...745..........*..%59..................726...........+.........249......../...*.....*..................%.........
    .....870................................978.......761=.......179.............692.........*..........477..550.....676..281.............507...
    ..............913.................215.........................*..745...#.............882..130..569....................&..........*472..*....
    ...27..710.....*.....................*.205*403..............59....+..172................=.............................................697...
    ....*....*....731.......865.......155..........849*....#..............................................679...854....................&........
    .985..%.............599....#..........191..520.......473..#........./...........708....406..677+...=............................599.....133.
    ......501...........*........@.........*...*..............446....767...*444........@......&.........49......#...311*413.....................
    ..................157.......830..13.90.398.945.978............................950.................%.......88................$...............
    .............297........618.......+.*............%............317.............../.........709..481............503.........570...305*845.....
    ......661....*....632.....@..+......524......../.....+...........*...590..............944....*...............*................+..........681
    .......*.....820...@..........829...........671.......206.......239.+......................339..#............106......415..213..........-...
    ........114........................903.669=............................764...319....373........642...................................-......
    .................*...........................*.......121.......898&.....$......*...*.....................................211#.......579.....
    ........*.......362......787.440..........842.825.....*......................590...357..........949......523....470..292....................
    .....594.459........98.....*....*.......................355&..*..................*..........554..*.......*.......*....*..222...243..........
    .........................535.....674.........804....#..........455.............847............=.32...23..951.....217.346....+.*....+272.....
    ..183...........443..........119.......833....#..693......818#........................@..699..............................*....678..........
    ...*......455/...%..826.............#.....*.........................$...........868.739....=..753...181....773.........323.508..............
    ...382................*...169..510..590.295....212.767....*970......711.........*........#.......*..*............651...........36..766*.....
    ............&..61..697.....*................46.*....*..691......970............786....953.....775...235..................*550....*.....417..
    ..........45.....*........689...645.....549*...821.601.............*......................952..............#..........464.....609...........
    .....657.........48...888.......*...................................305......+47..........................254.27......................123...
    818......388............*....412....../491.........#336...506..422.................989....15...70..............*..354............@.....*....
    ..............506........11......./...........643..........#..=..........703...963*........@.........613.....413....%...485.......265...33..
    ....589.......*................207..............%...............................................+................*......*.....94............
    ...*.........663...413+....................................819...........655.%...648..823.......770....385......457..150...#.../............
    515.......................475....825......33../350.....459...%..........*....681...*...*..............%..................437.......689......
    ....$993..897@..............*.......*837...-...........*..............726........372....922......&198................................*......
    180............*..........679.................71.629=.796......109...........................527..................682.....558.....798.......
    ........437.742.46.................@.......&...................*......498.......@.............*............./......$........*...............
    ..761..$...........148.......856....701.377..536*510...604...540.....&........929........172.799.............310.....700...........440.%....
    .....*........808*....=.........@......................*.........577.....................*................&...........$...417.39.....&.229..
    ........794.......446..............................214.543...%...*..........935......865..445...../....%.615................=..*............
    ..572..+...............................419....932@.........924..72.902.......-..579.-...........997.203................873......945..567....
    .....*.................273.............*...........................*...142............688................755.850...........846.........=....
    ...880................*..............883.698.....$.......=........197.*...........316*.........365.........$...*............*............904
    .........385..977..179....+...................652......875.............350...863............73......464.........214.852..478................
    ......22....*..@.........93............*682..................*137..309.........*....557.....*........................=.......698.....+.218..
    ......*...645......163...............89......*853..................*...386..341.......*...228............718*492...............*..153..*....
    .....576......................857.........513....................509......*.....322.710.........615.......................613.165.......292.
    ................................@.....370.......598......487...........503..*.....................*...738#..#..$901...$.....#.......648.....
    ..19*454.............78................/..876..#..........*......+..........937.......383........523........69......326.678.....212....*....
    .....................*.....436.@...646...*..........100....367..325.....361.......207*....50............621...............*..........237....
    757.........714......502...%...54......909.411.....*.....................*..........................=..$................415.................
    ...$.......&.................................*...471..985...508.472....891.842..$.................418.........&.....&.......................
    .................$........906..........*215.338...............*..............*..606.#235......................813..259........229...........
    ........472...222.........*.....245.572.......................927...769....344.............291.239.....................94....*..............
    ...818...$.............291..............288...........163.734......+...........699............*.......981&...514*.............224.....=.....
    .....*..................................*.........................................+..189..........452............644...................669..
    ....850................326$..............869..&985......784..../..............592.......*..153.....*....887..493.....211........%...........
    .............................457....834.....................94..564..........&.........498....*...304....../...*.....*..........845.....&...
    .......12*48.753...244..196....=......$..721....90*29.........*.................+.522.......487...............317.....531..311........20....
    .177...............*.....*.................*.............*969.611.......*565..338...*.............712..922$.......770......*....522.........
    ....*..459......149......165...=........879...........103............497..........258.931..75....*..........400*.....*......972.............
    .269...*............#........715.............................*475..........................*....145.............414.....228........893......
    ........209......508...................../331......469....894............347.890$..541.....56...............#........#..*...................
    ......................=...........37................*...........................................$...........197...119.....=...........756...
    ...........&.......850.....367.34...*............517........178*716..........=...................892......-............883...755............
    .....@496.125.................*......965.....428.....908..................405.....650...................=..493..89..........................
    ..............581........396.............596.........*....964........672......159.%....463............768......./.............391...........
    .............*...........*.....871.660..*......577.73......@.....932....#....*............*574...186........154......996........*......720..
    ............914..132..137......=.......746......*..................*......376.......&661............*560.%.......%....*...264....511........
    ...%...151................*851...................................*..284.........754..........190.19.......611...326.841...............89....
    644.....*...797........133.....476..............................810.............*.....866......+.*..............................#903..*.....
    ........839..*..478...........*........372.#.....744.....227*............477.3.253.%....&..746.....289.&860.....720*708.....................
    ............443...%.....68*....283.......%..900...%....%..........584......*.$......308....*......*......................592................
    ...239................@....250........50........&...%...192.$111.*........74............831....914...#.....886*492..........=...............
    ...*......425.....181..680..............*......873.502............93.................................503.@..............116........372......
    492..........*.....$.............302...789.................861.......................290.......452........83....626....*.....76.......*.....
    ......178...528..........*739...@..........................*......804....320....#.........&878.*.....746........./..&...148...$......741....
    ...&....*.............262..............325...........599..592.579.....%...+..808...............386..*...............761.....................
    663..462...........*..........109..706*.................=.............553........712......*971......674.396...635*..........................
    ........../...728.952...413......*......744.......%..........................300..*....782................*.......742..&424........41+...564
    ........375...%.........*......450.456.$.........714........851.327..#...+......*...+.......179.630....854..................................
    ..251................195...887.....*........*209.......430.*......-.83.596....956..966.274.%.....*........................83.359........411.
    ....%.......................*..........89.........194..+...491.........................&.....*.762..+741..........598.......*.....910..*....
    ..........618+..............677........*..598........+..........-.....58....290...........795...........................*.....580.&....559..
    ...................561..............910......-..321....834....602.752*....=..+........191..............463....@..........792..*.............
    ............216.......%...722*668.....................$..................109............*....222...195*.......689....380.......637..........
    .....846.......*..436..............=827...263@.764*.......*.464.................729.....934...*......................................289+...
    .127*.......757.....&......./954...................965..664........./......716*.+...........266....781.........989..........844...$.........
    ................57.....................&118......=...........385.623...245..............345........*.....615-.....*..........*...768........
    ...538...........#..860......................783.460..........*............401.............*796..354..............37......797........725.193
    ....*..#620../.........*..........578..54...*..........780..251...634..946....*........................./....702........................*...
    ..202.......815........724....#..............822..972...................$..........*.....#....@........27.....*...........268.....347.......
    .................+..........98.....703..............&....*52.....320.............670...318....734.............47..................+....239..
    .953*919...=......273................=...454.....@....601...........*...958.........................+.................758...=...............
    ..........905./50.........@...................971..................762.*..................169........915.682..........=......533.......502..
    .....884.................24...........262.........@.....531..698.......234.................................................%.......149..*...
    ....................834.....994.......*........266.....*......*............................................/.....941.434...812.......*..211.
    .............133......*......*.....613...85..........871.......497..346...737....88.....176.....192$........128.*.......&......./163.26.....
    ....734..543....*.....656....461........*......531..................*........../...*..&................$971.....931.........................
    ................606....................506............................779.......30...211.....243..........................153...504.........";

    let mut l: Vec<&str> = input.split("\n").collect();
    let ll: Vec<Vec<char>> = l.iter_mut().map(|li| li.trim().chars().collect()).collect();

    let mut number_flag: String = String::new();
    let mut is_adjacent_flag: bool = false;
    let mut gears: HashSet<(usize, usize)> = HashSet::new();
    let mut all_gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut total = 0;
    for (j, row) in ll.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if *v == '.' || i == 0 || check_if_symbol(*v).0{
                if !number_flag.is_empty() {
                    if is_adjacent_flag == true {
                        println!("{}", number_flag.parse::<i32>().unwrap());
                        let n = number_flag.parse::<i32>().unwrap();

                        for gear in gears {
                            match all_gears.get(&gear) {
                                None => {
                                    all_gears.insert(gear, vec![n]);
                                }
                                Some(g) => {
                                    let mut h = vec![n];
                                    h.extend(g);
                                    all_gears.insert(gear, h);
                                }
                            }
                        }
                    }
                }

                number_flag = String::new();
                is_adjacent_flag = false;
                gears = HashSet::new();
            }

            if v.is_digit(10) {
                number_flag.push(*v);
                
                if is_adjacent_flag == false {
                    is_adjacent_flag = check_adjacent(&ll, i, j);
                    gears.extend(check_adjacent_gears(&ll, i, j));
                }
            }
        }
    }

    println!("{:?}", all_gears);
    for elem in all_gears {
        if elem.1.len() == 2 {
            total += elem.1[0] * elem.1[1];
        }
    }
    println!("{}", total);
}

fn day_three_one() {
    let input = "12.......*..
    +.........34
    .......-12..
    ..78........
    ..*....60...
    78..........
    .......23...
    ....90*12...
    ............
    2.2......12.
    .*.........*
    1.1.......56";

    let mut l: Vec<&str> = input.split("\n").collect();
    let ll: Vec<Vec<char>> = l.iter_mut().map(|li| li.trim().chars().collect()).collect();

    let mut number_flag: String = String::new();
    let mut is_adjacent_flag: bool = false;
    let mut gear_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut total = 0;
    for (j, row) in ll.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if *v == '.' || i == 0 || check_if_symbol(*v).0{
                if !number_flag.is_empty() {
                    if is_adjacent_flag == true {
                        println!("{}", number_flag.parse::<i32>().unwrap());
                        total += number_flag.parse::<i32>().unwrap();
                    }
                }

                number_flag = String::new();
                is_adjacent_flag = false;
            }

            if v.is_digit(10) {
                number_flag.push(*v);
                
                if is_adjacent_flag == false {
                    is_adjacent_flag = check_adjacent(&ll, i, j);
                }
            }
        }
    }
    println!("{}", total);
}