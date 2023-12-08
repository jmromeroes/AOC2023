use std::{collections::{HashMap, HashSet}, cmp::Ordering};

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
    // day_three_two();
    // day_four_one();
    // day_four_two();
    // day_five_one();
    // day_six_one();
    // day_six_two();
    day_seven_one();
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

fn day_four_one() {
    let input = "Card   1: 30 48 49 69  1 86 94 68 12 85 | 86 57 89  8 81 85 82 68  1 22 90  2 74 12 30 45 69 92 62  4 94 48 47 64 49
    Card   2: 57 32 92 73 76 62 11 19 61 90 | 19 82 53 87 57 80 69 76 90 56 11 61 30 92 73 99  4 32 33 64 34 62 27 78 65
    Card   3:  6 56 40  1 47 26 25 87  4  2 | 12 26 32 25  8  4 41 54 69 99  2 45 70  6 59 23 47  7 49 17  1 56 92 87 40
    Card   4: 26 49 44 82 25 43 47 74 97 13 | 76 62 13 82 55 26 93 84 83 19 47 22 49 44 25 43  7 18  9 45 97 15 90 85 74
    Card   5: 88 58 96 80 56 16 55 13  3 40 | 20 57 23 71 76 43 36 72 52 18 60 28 80 84 64 75 93 46 19 69 25 31 58 45 47
    Card   6: 89 88 34 62 60 41 15 42 57 58 | 58 49 82 42 70 78 72 57 77 47 62 89 30 60 96 98 54 66 25 14  6 34 15 41 88
    Card   7: 97 73 29 26 91 16 66 64 70 23 |  3 12 61 11 70 59 71  1 26 44 14 64 63 93 20 17 89 16 43 96 52 92 46 97 85
    Card   8:  9 33 46 87 34 97 71 30  5 43 | 21 64  8 43 77  9 20 46 10 19 53 33 71 56 39 87 34 50 72 91 30 89 13  5 97
    Card   9: 72 41 12 58  8 56 11 82 22 66 | 27 97 48 14 77 37 33 91 85 39 75 42 58  2 52  9 70 45 72 62  5 21 23 60 99
    Card  10:  3 46 56  2 58  4 92 80 86 52 | 15 56 91 57 61  8 87 34 51  3 23 41 96 45 48 49 43 25 26  9 58 50 52 38 12
    Card  11: 76 94  4 55 97  8 22 99 80 70 | 64 75 23 49 42  1 66 54 85 29 28 94 22 93  4 69 72 62 41  2 86 83 97 34 82
    Card  12: 65 91  7 24 80 20 46 29 58 57 | 51 59 52 63 28 44 62 35 46 49 24 14 58 91 65 78 84 20 43 32 23 15 29 13 22
    Card  13: 98 94 55 43 83 72 19 46 45 22 | 55 67 59 91 46 37 24 52 98  4 72 22 44 94 88 43 27  7 75 13 45 82 15 65 19
    Card  14: 92 51 81  6  5 91 73  9 80 94 | 56 72 71 32 92 98 15  2 10 91 62  1 51 74 69 89 33 96 39 97 21 86 65 52 83
    Card  15: 84 40 87 91  3 61 20 73  2 37 | 77  3 37 81 20  9 51  2 87 73 53 66 26 36 69 99 83 96 88  5 91 18 40 57 80
    Card  16: 97 63 52 67 86 87 91 25 69 43 | 68 67 75  3 85 10 88 99 52 43 95 92 80 87 11 13 96 58  1 17 74 20 36 93 77
    Card  17: 28 90 77 78 52  7 94 13  6 88 | 11 69 49 56 59 14  7 24 53 21 67  2 17  8 47 66 48 30 79 43 86 64 46 28 23
    Card  18: 50 22 25 69 54  4 37 14 49 75 | 82 84 78 88 87 15 80  1 11 91 89 93 25 98 74 42  6 59 23 58 13 46 49 32 95
    Card  19: 96 56 86 38 29 43 69 66 26 46 | 80 13 90 30 41 86 44 95 71 23 40 75 33 14 56 59 89 18 92 79 15  4 84 68 82
    Card  20: 95 26 31 69 63 25 56 87 74 48 | 88 16 42 22 30 14 71 62 35 87 99  3 96 54 41 83 40 29 79 20 50 24 53 10 66
    Card  21: 75 76 94 26 24 56 34 35 93 87 | 42 23 71 55 44 79 12  4 78 32 82 53 40 35 43 50  9 65 29 57 93 86  7 67 19
    Card  22: 53 58 25 33 42 98 94 95 97 22 | 44 91 40  1 88 31 61  6 54 52 47 93  3 99 27 59 15 53 56 39 66 43 51 41 48
    Card  23: 34 95 33 45 56 37 30 94 76 67 |  9  1 72 68 51 66 36 24 21 92 35 50 62 86 90 49 82 46 15  4 70 40 75 64 73
    Card  24: 73 93 49 22 86 18 31 87  3  2 | 27 42 46 97 49 31 47 25 93 74  9 60 44 87 15 63 18  2 73 89  3 62 86 65 22
    Card  25: 36 24 95 21 31  7 56 93 82 17 | 82 36 23 54 66 83 95 31 65 16 17 86  7 21 61 24 55 56 13 53 59 64 96 39 32
    Card  26: 16 89 92 12 40 17  3 80 20 21 | 37 72 26  9 79 55 74  2 92 59 17 35 71 94 98 31 58  3 32 51 21 88 40 96 57
    Card  27: 70 19 81 48 62 26 45 86 65 41 | 63 38 71 43 50 42 25 14 18 54 66 96 29 36 95 73 20 76 16  6 58 78 56 87 91
    Card  28: 61 66 57 75 52 20 41 21 24 14 | 83 51 77 14 66 91 75  3 47 20 18 65 76 52  5 11 72 57 10 36 30 92 73 28 19
    Card  29: 94 39 68 79 28 70 81 97 59 48 | 46  4 41 39 38 75 18 76 45 31 79 97 91 64 71 85 63 55  8 94 67 59 36 74 49
    Card  30: 90 45 15 47  1 24 99 62 69 55 | 42  5  1 37 77 71 66 57 94 18 26 47 46 27 48 80 24 95 30 15 53 90 38 99 69
    Card  31: 53 16 59 23 78 95 38 17 40  7 | 86 20  7 98 44 19 73 36 40  4 58 16 94  2 83 32  6 78 57 13 35 30 87 51  5
    Card  32: 64 79 78  6 24 15 44 45 82 33 | 34 18 80 94 54 59 14 23 77 25 73 81 26 62 17 45 37 82 67 24 98 78 85 43 13
    Card  33: 91 97 83 95 89 32 86 94 96 19 | 93 39 57 58 41 62  1 80 33  2 24 51 27 25  6 10 78 76 63 65 38 79 61 98 90
    Card  34: 41 84 69 81 95 71  7 80 65 55 | 88 17 73  3 54 66 68  8 76 84 36 79 55 47 58 91 37 53 57 46 93 71 83 42 12
    Card  35: 51 67 16 48 34 86 59 74 98 65 | 14 75 23 98 48 83 11 35 18 93 68 50 85 31 40 82 27 92 55 63 64 37 76  2 33
    Card  36: 26 80 18 67 87 93 79  1 59 61 | 83 51 97 34 70 90 49 44 39 95 92 38 55  9 65 96 94 82 61  1 20 46 50 63  6
    Card  37: 71 13 91 66 24  3  1 41 27 75 | 12 38 45  5 80 11  8 59 90 64 85 49 52 56 35 96  7 20 53 39 44 22 76 57 66
    Card  38: 21 56  2 42 51 28 37 38 95 12 |  3 84 55 81 91  6 90 14 68 93 64 33 78  7  4 60 18 54 11 29 26 19 75 72 52
    Card  39:  1 40 97 60 90 47 74 41  7 58 | 43  8 47 72  7 78 41 96  5 37 98 97 58 89 23 65 74  1 92 90 28 16 60 40 67
    Card  40: 75 99 14 36 64 42 30  8 52 97 | 64 89 36 80 99 30 57 40 35 75 53 97 52 38 68  1 74 42 14 60 49 79  8 72 20
    Card  41: 26 38 87 47 39 62 56  2 49 51 | 25 45 87 63 24  2 39 51 54 56 70 62 77 57 68 49 31 34 26 72 42 38 47 94 29
    Card  42: 26 69 10 47 89 27 43 62 97 50 | 62 79 50 69  2 97 68 87 27  5 67 18 64 26 10 25 91  4  7 43 70 14  1 54 89
    Card  43: 92 16 95 21 15 77 27 88 76 63 | 92 77 78  1 58 96  8 31 63 16 14  2 54 88 15 52 73 76 21 27 84 71 25 95 72
    Card  44: 27 31 64 16 96 95 74 57 59 53 | 75 20 48 11 18 44  3 45 79 98 12 54 30 60 32 15 13 72 21 87 85 52 70 33 35
    Card  45: 62 31 89 91 85 83 93 11 26 23 | 53 55 71 62 14 65  8 21 96 57 17 91 51 82 86 47 64 80  3 69 56 90 85 32 70
    Card  46: 52 67 93 77 39 13 55 63 68 78 | 19 96 76 50 11 61 72 81 64 10 84 22 83 37  3 58 80 48 57 18 59 62 92  4  9
    Card  47: 31 14 25 56 83 48 53 64 39 77 | 48 34 27 16  5 55 77 42 17  6 24 49 37 92 51  8 97 52 41 11 31 80 47 87 38
    Card  48: 69 65 89 40  1 31 64 50 48 38 | 44 75 74 58 13 56 50 80 22 89 47 95  8 53 31 38 48 64 72 39  1 52 43 17 65
    Card  49: 30 49 86 87 78 96 73 44 21 90 | 14 39 75 49 96 63 56 73 64 81 93 98 99  1 13 65 54 88 61 83 44 30 45 15 67
    Card  50: 61 79 65 67 77 58 21 37 90 48 | 81 79 62 51 21 85 61 90 92 47 29 23 80  7 98 40 59 87 18 77 48  6 99 88  1
    Card  51:  9 45 35 38 92 47 27 61 91 67 | 75 86 50 74 41 92 59 71 52 91 97 73 96  4 19 87 69 76 98 21 90 37 32 14 77
    Card  52: 82 80 79 33 18 70 52 73 38 39 | 21 82 95 25 66 89 75 85 84 39 47 28 49 45 79 17 56 10 29 52 44 77 70 32 65
    Card  53:  2 47 76 33 17 62 13 70  9 31 | 84  2 31 47 13 97  4  3 33 21 17  8 62 10 19 78 41 70 43 98 79 59 95 29 99
    Card  54: 50 10 89 77 41 82 43 84 30  7 | 28 49 35 76  9 23 65 71 60 36 32 15  3 61 95 10 24 77 17 97 48 62 88 51 68
    Card  55: 98 86  8 43 54 59 11  7 63 42 | 24 98  1 58 51 68  2 56  8 54 29 25 66 17 59 86 30 48 43 60  5 55 18 44 28
    Card  56: 88 38  8 43 91 12 69 45 71 20 | 11 34  1 95 77 19 39 40 33  5 80 51 75 94 84 23 82  8 86 62 74 89 96 22 66
    Card  57: 95 38 81 99 57 68 60 55 39 18 | 34 38 45  7 63 12 97 35 44 17 57 58 92 41 89 30 39 47 66 10 82 52 68 55 98
    Card  58: 34 72 18 15 20  1 28 89  7 35 | 14 22 61 36 84 26 55 62 32 53 44 51 46 17  6 54 85 35 95 40 10 92 42 19 83
    Card  59: 58 43 86  3 73 17 24 49 29  5 | 87 93 56 60 35 48 86  8  7 16 67 89 73 43 69 31 88 23 50 59 45 84 61 26 65
    Card  60:  4 74 59 28 17 23 67 48 82 20 | 76 91 88 47 84  2 14 61 56 11 58 34 24 99 75 66 33 96 92 94 64 73 72 45  7
    Card  61: 24 40  2 95 58 17 28 43  4  8 | 97 77 49 15 51 41 70 80  6 74 11 68 38  5  8 64 81 30 89 72 63 52 39  7 82
    Card  62: 79 22 90 64 37 95  4 87 30 38 | 73 25 91 13 83 45 77 53 88 63 43 92 78 34 32 58 24 40 50 61 55 41  2 67 14
    Card  63: 70 13 43 35 58 84 44 51 65 73 | 43  3 48 51 69 70 31 98 19 15 91 61 55 39 41 58 44 65 35 13 25 73  9 50 56
    Card  64: 85 64 44 45 31 95  3 16 19 74 | 53 13  9 89 83 58 47 90 77 59 98 99 74 91 52 15 61 80 75 88 54 23 24  8 29
    Card  65: 70 33 34 72 80 56  7 95 71 32 | 16 32 56 34 73 31 45 29 71 33 98 72 97 19 39 94 91 90 70 75 40  7 95 80 96
    Card  66: 29 30 35 39 52 17 25  1 62 79 | 83  2 69 13 36 76 59 50 90 64 30 37 48 75 95 25 26 60  7 29 86 42  8 22 11
    Card  67: 51 59 72 52 60 74 20 68 90 40 | 51 34  8  2 67 46 47 10  9 43 61 56 28 74 73 60 81 25 87  4 17 84 65 42 32
    Card  68: 36  3 98 39  7 74 66 85 81 14 |  7 78 14 74 85 97 55 62 66 98 28 94 39 81 42  6 24 36 60 50 75 87  3  9 18
    Card  69: 72 99 53 87 83 54 58 32 79 26 | 54 70  5 34  1 79 26 99 90 11 77 13 98 83 15 58 40 96 91 12 32 53 87 72 43
    Card  70: 72 26 89 44  9  7 14 95 46 51 | 45 44 91  3 71 77 82 56  2  6 24 20 49 75 60 22 58 80 42 68 32 54 76 29 79
    Card  71: 56 75 58 97  4 95 16 23  7 71 | 63 96 40 64 42 38 65 48 78 35  5 24 20 41 70 89 45 17 49 21 10 44 92 60 51
    Card  72: 82  3 37 72 19 78 84 69 43 62 | 95 43 97 82 72 14 70 47 45 26 92 77 67 62 49 22 19  3 37  7 69 78 94 84  6
    Card  73: 63 77 50 12 46 80 13 54 64 24 | 60 50 17 87  1 46 42 13 80 63 82 48 45 35 55 29 30 31  9 24 64 27 77 40 69
    Card  74: 66 73 38  5 89  1 11 10 91 92 | 89 23 28 50 82 63 33 91 72 74 73 66 24 17 42 11 36 94 79 92  4 85 29 57 10
    Card  75: 66 51 34 46 78 27 89 42 52  4 | 87 79 28 18  1 35 81 60 65 76 56 78 42 43 47 45 89 70  4 68 19 63 33 94 86
    Card  76: 35 43 54 87 15 71 30 92 29 24 |  6 76 36 34 50 47 88 29 44 73 68 97 87 24 81 20 94 86 70 90 71  2 15 98 54
    Card  77: 27 77 81 31 53 47 45 71 73 41 | 85 40 63 68 22 10 75 55 62 67 54 49 51 79 17 92 93 21 73 38 37 74 90 23 98
    Card  78: 79  1 93 23 31 48 32 53 64 57 | 52 76 35 68  3 75 22 81 79 51 80 46 88 65 83 44 67 89 31 86  8 98 97 16 30
    Card  79: 72 80  7 74 10 22 12 34 89 97 | 47 62  5 23 17 27  9 21 78 68 66 98 29 39 51 83 24 90 64 10 48 22 97  2 35
    Card  80: 80 94 30 12 28 54 78 34 58 63 | 46 33 86 44 50 94 96 66 57 58 87 65 92 71 32 56 73 90 11 85 21 76 45 27 64
    Card  81: 94 60 72 30 14  3 40 86 69 82 | 74 99 11 55 27 54 70 46 89 24 44 85  6 53 58 15 26 10 20 38 56 63  1  4 12
    Card  82: 88 30 84 71 61 63 20 56 49 89 | 10 87 14  2  9 57 46 27 12 41 78 59  5 55 19 94  7 24  1 25 13 28 42 33 68
    Card  83:  6 35 37 64  9 44 21 42 56 30 | 57 69 83 89 47 74 95 71 81 84 41 45 40  2  1 77 32 75 90 60 62 82 79 22 55
    Card  84: 34 13  4 21 88 26 18 96 38  3 | 16 66 63  9  7 95 28 72 74 41 10 85 30 70 91 69 47 44 33 49 60 46 57 62 36
    Card  85: 94 58 24 74 61  5 49 99 30 54 | 99 11 61 58 25  4  2 55 30 75 62 51 94 22  3 36 54 84 71  9  8 24 70 90  5
    Card  86: 33 82 19 66 52 24 76 34  5 84 | 82 37 63 83 36 22 13 76 84 19 38  6 65 33 66 40 95 52 24 45  5 88 34 89 49
    Card  87: 86 24 92  7 72 28 63 23 12 82 |  9 59 36 56 40  1 76  4 88 10 85 14 17 15 61 29 57 50 98 49  3 41 42 93 22
    Card  88: 23 32 78 34 12  8 89 20 50 29 | 59 24 13 72 14  7 83 37 32 39 90 46 96 49 94 82 43 33 85 92 69 62 57 48  9
    Card  89: 98 48 90 71 56 21 61 86 29 63 | 50 62  2 23 70 44 36 35 57 48 13 64 21 90 93  1 53  5 33  7 29 67 38 61 54
    Card  90: 27 66 35 40 54 68  1 80 42 49 | 97 94  2  1 66 55 15 49  5 75 26 30 20 25 73 79 10 40 57 85 23 43 68 38 14
    Card  91: 52 74 25 22 84 68 76 27 89 13 | 52 75 90 22 73 94 64 72 84 70 74 67 30  2  1 29 54 86 25 58 41  6 43 91 34
    Card  92: 35 83 75 46 37 28 82 54 13 77 | 96 77 46 33 37 59 62 13 28 54 75 86 74 35 47 85  6 80 55 20 36 83 38 82 65
    Card  93:  8 19 27 84 17 42 23 22 78 16 | 76 16 63 78 89  1 27 57 83 34 36 69 42 95 41 44 74 28 79 39  4 92 19 23  7
    Card  94: 90 53 59 26 63 91 50 81 69 85 | 20 78 12 23 31 91 42 36 75 95 19 76 96 49 94 47 63 53 27 88 67 69 85 60  7
    Card  95: 87 25 89 31 63 32 13 80 60 14 | 95 82 78 34 87 13  3 60 88 28 96 59 36 77 14 79 55 35 83 25 73 58 48 63 86
    Card  96: 32 27 21 77 35  9  8 28 97 82 | 53 97 29 81 66  3 17 85  8 35 56 38 26 60 49  9 39 86 28 71 89 88 54 44 27
    Card  97:  2 42 48 43 96 55 51 60 33 89 | 85  3 31 88 98 25 41 81 44 68  4 80 63 17 45 57 89  7 38 76 95 53 49 67 13
    Card  98: 63 57 83 58 23 27 92 21  2 76 | 58 76 95 80 62 56 98 33 59 46 51 23 93 69 96 48 36 53 19  8 45 32 42  2 89
    Card  99: 28 85 31 55 69 73 70 66 11 79 | 22  4 83 55 26 20 29 72 54 12 30 99 66 14 10 17 15  8 88 45 76 69 58 89 75
    Card 100: 99 91 55 84 81 61 94 95  8 25 | 30 15 52 88 27  6 53 41 45 95 66 19 98 86 26 31 54 43 51  9 82 97 57 60 16
    Card 101: 86 47 76 89 85  1 34 35 53 51 | 81 13 69 14 79 68 37 39 63 59 50 15 56 99 92 80 36 43 18 77 65 19 45 91  8
    Card 102: 98 27 49 51 14 77  6 41 97 64 | 60 76  3 79 48 42 29 37 10 24 74 62 11 17 95 73 56 85 28 38 90 63 84 30 94
    Card 103: 75 45 64 88 40 69 96 66 52 28 | 28 12 88 45 60 64 16 96 41 17 82 97 24 75 54 67 92 14 40 85 93 52 31 37 66
    Card 104: 36 69 82 51 81 42 92 95 62 73 | 13  5 65 87 70 36 95 99  9 62 24 55 31 42 51 82  7 34 69 92 14 18 79 81 73
    Card 105: 87 81 71 62  3 41 17 86 88  5 | 98 31 80 39 25 35 47 42 95 14 91 83 65 12 87 96 88 58  5 19 56  1 97 62 48
    Card 106: 31 55 80 86  7 20 37  2 89  4 | 70 20 92 62 38 86  7 93 79  6 48 37 19 21 89 47 25 78 43 46 54 69 63  8 80
    Card 107: 99 85  5 37 34 95 11 12 51 96 | 69 64 67 53 15 81 89  3  2 54 49 98 71 72  1 76 91  8 17 51 23 83 21 86 92
    Card 108: 95 57 98 30  3 83  6 64 52 18 | 59 88 91 78 61 30 52 98 65 68 25 17 13  3 64 89 57 37 83  6 95 36 97 20 18
    Card 109: 97 99 89 96 69 84 62 66 61 83 |  9 31 22 49 96 84 17 61 15 99 39 89 26  4 92 32 97 21 62 56 66 82 69 83 13
    Card 110: 61 93 64 41 91 76 74 21 56  5 | 51 65 17 29 19 45 70 87 40 10 62 20 89 85 53 36 42 37 50  2 74 56 64 46 48
    Card 111:  8 85 35 16 52 31 94 10 29 82 | 70 60 11  6 31 58 10 52 43 35 16 30 94 62  1 29 74  9  4 90  7 59 40 20 85
    Card 112: 96 15 95 19 76 41 77 62 46 89 | 61 35 99 56 60 82 17 16 47  4 67 20 91 79 78 97 40  7 13 43 87 74 34 31 84
    Card 113: 28 83 44 97 18 22 96 49 17 98 | 86 88 11 92  5 54 90 51 38 52 36 13 64 81 45 63 85 15 47 40 18 50 67 39 73
    Card 114: 89 28 69 44 47 58 20 60 25 92 | 97 61 98  4 37 30 68 96 86  6 50 34 71 53 78 57 10 36 35 40 67 33 93 17 75
    Card 115: 20 49 40 31  8 55 28 79 99 22 | 95 87 21 14 62 17 70 81 83 54 96 39 18 35 44 61 23 30 80 89 42 50  7 58 47
    Card 116:  3 75 52 97 60 24 50 13 54 71 | 17 21  1 98  8 79 59 88 97 53 82 28 34 45 93 69 13 85 38 26  7 37 43  5 22
    Card 117: 44 70 59 87 64 20 17 19 68 71 |  6 46 47 79 35 74 78 26 19 99 20 80 31  1 68  3 12 39 59 62 94 25 91 92 88
    Card 118: 65 74 73 48 97 21  1 43 57 29 | 73 25 58 67 56 41 19 20  1 12 99 48 47 96 68 50 88 49 80 10 39 75 11 82 79
    Card 119: 68 40  2 54 25 75 77 14 50 86 | 27 69 39 90 81 15 17 54 63 51 87 52 37 22  7 38 11  8 24 96 42 60  6 99  4
    Card 120: 43 63 34 38 57 39 23 29 16 41 | 90 93  1 72  9 80 94 73 28 65 87 25 78 33 98 66 52  2 36 30 92 54 47 24 12
    Card 121: 92 44 29 97 22 41 73 16 31 65 | 20 51 50 43 30 27 39 40 88 14 57 77 62 90 18 87 26 54 95 48 34 10 93 47  9
    Card 122: 61 12 44 52  4 69 42  1 77 54 | 24 23 11 39 78 52 12 69 73 42 47 43  4 29 37  1 36 77 19 61 95 54 94 44 28
    Card 123: 17 44 12 66 30 19 96 54 34 75 | 24 17 42 90 14 92 84 35 71 12 34 32 99 19 30 98 66 70 54 44 56 96 77 36 75
    Card 124: 99 74 53 87 15 93 58 96 24 44 | 15 58 96 29 49 50 87 93 86 53 12 51 24 44 60 74 19  8 67 99 76 72 45 80 81
    Card 125: 28 95  5 13 21 49 70 33 48  7 | 61 57  3 65 89 13  7 70  1 95 84 33 46 21 28 79  5  9 19 48  4 76 82 86 49
    Card 126:  1 83 21 50 35  3 55 84 38 81 | 61 80 30 26 83 84 38 92 73 21 42 14  1  6 81 15 50 57 55 56 35 63 67  3 93
    Card 127: 37  8 99 82 24  6 62 63 94 29 | 62  8 83 82 24 87 46 55 78 91 31 23 29 16 99 63 54  9 68 48 94  5 37  6 40
    Card 128: 18 47 91 78 74 63 49 42 85 10 |  4 10 82 20 97  3 71 74 62 95 60  9 28 73 47 32 14 18 49  7 83 92 85 53 78
    Card 129: 19  9 11 30 91 83 99 93 46 22 | 38 33 91 85 57 31 15 99 97 54 17 50 71  8  6 29 59 36 75 81 82 67 44 65 25
    Card 130: 62 13 98 39 78 28 63 38 41  6 | 51 99  1 98 81 40 39 63 62 52 25 47 55 13 34 41 38 18 14 73 97 28  6 48 78
    Card 131:  9 69 43 61 65 35  4 59 71 96 | 17  4 65 29 19 48  7 32 69 61 23 15  1 37  9 80 59 47 64 41 35 79 71 43 96
    Card 132: 40 62 73 49 19 51 30 95 92 55 | 77 79 31 51 92 41 49 27 13 40 99 52 68 98 32 26 29 20 60 25 65 72 11 70 19
    Card 133:  7 73  1 68 16 40 10 37 78 91 | 39  9 33 97 98 13 35 38 72 75 82 37 27 32 93 48 87 50 94  5 95 67 46 24 54
    Card 134: 43 78 49 90 73  7 38 95 12  4 | 10 79 30 12 25 39 32 68 95 78 45 40 73 37 70 49 41 46 91 42 23 18 76 15 90
    Card 135: 83 66 13 30 47 75 52 69 17 48 | 76  6  7  1 12 64 89 62 52 59 83 97 33 28 66 20 72 13 84 65 56 73 53 92 30
    Card 136: 63 90 52  1 43 85 83 22 47 57 | 17 44 77 36 79 20  1 65 29 50  6 81 78 76 70 97 90 68 67 34 31 82 88  2 25
    Card 137: 77 25 91 14 87 80  8 90 27 57 | 91 63 29 60 71 57 98 14 52 77 39 96 85 50 76 10  8 27 32  4 23  3 17 62 24
    Card 138: 33 17 89 10 85 90 79 72 13 53 | 30 45 81 89 47 57 32 80  1 82 84 97 16 90 66  4 75 94 29 54  9 78  3  8 93
    Card 139: 57 99 35 94  5 11 86 67 34 96 |  1 79 54 17 20 82 26 32  7 77 44 51 18  3 76 64  9 72 35 83 37  5 60 66 22
    Card 140:  6  9 44 98 64 92 30 14 62  4 | 43 49  9 79 42 27 20 69 36 24 47 19 14 38 98 54 77 73 41 56  4 15 86 10 44
    Card 141: 28 99 14 96 19 92 12 23 17 22 | 64 19 85 70 80 16 63 96  2  3 61 67 41  9 74 12 37 30 86 36 26 92 94 66 71
    Card 142: 28  2 68 35 71 10 98 57 79 14 | 44 78 23 24 59 49 21 47 75  6 60 40 22 15 46 41 71 53 87 90 32 52 35 72 37
    Card 143:  8 10 60 94 66 51 36 29 56 69 | 71 69  2  9 12 55  1 53 91 84 22 83 77  6 18 57 10 24 44 72 79 73 95 62 99
    Card 144: 21 12 42 22 50 10 88 18 19  9 | 25 72 96 47 60 85 23 10  7  3 26 81 59 80 38 91  5 70 95 97 53 33 67 74 92
    Card 145: 53  5 14 26 75 35 21 39 30 43 | 13 18  6 46 69 76 78 67 81 68 36 83 91 97 32 40 23 92 29 28 42 51 71 20 37
    Card 146: 18 66 74 11 31 91 15 55 95 87 | 54 74 55 18 89 28 95 72 62 38 15 92 91 31 44  9 81 87 88 66 24 23 71 16 11
    Card 147:  7 21 81 38 56  2 55 50 60 83 | 15  9 91 69 44 92 54 99 72 43 59 78 25 79 32 51 64 20 62 24 75 38 29 16 97
    Card 148: 46 52 47 60 29  9 67 22 10 23 | 92 82 54 91 74 55  1 73 26 61 64 90 28  2 72 40 27 32 51 20 89 31 75 97 99
    Card 149: 27 58 78 77 96 68 54 89 31 86 | 48 42 86  8 21 20 60 76 72 37  2 75 33 67 57 28 41 78  1 49 95 87 38 94 62
    Card 150: 38 19 90 28 21 59 10  7 66 80 | 68 60 40  3 50 55  6 65 64 59 47 20 56 70 14 29 26  4 22 74 17 69 46 75 87
    Card 151: 93 89 25 22 91 70 10 78 84 50 | 21 35 78 58 30 71 11 70 99 84 97 87 57 67 15  9 20 91 79 50 26 74 47 56 12
    Card 152: 68 34 95 27 26 87 56 85 92 78 | 58 33 87 81 64 95 27 48 85 72  6 62 65 50 39 96 37 92 28 80 60  1 83 31 74
    Card 153: 34 11 38 50 57 78 52 73  7  4 | 56 78  9 11 32 92 26 86 40 27 24 91 18 13  4 77 37 61 25 58 60  8 30 16 80
    Card 154: 40 88 81 92  6 33 60 25 98 37 | 77 28 68 92 73 41 81 24 40 67 11 44 17 91 82 39 69  8 57 62 29 76 99 47 79
    Card 155: 15 95 17 21 76 78 69 54  1 40 | 68 17 89 96 23 49 20 53 47 93 37 55 10 69 21  4  3  6 75 65 95 79 67 30 91
    Card 156: 82 50  7 62 47 55 85 26 41 17 | 71 94  8 43 83 90 76 20 66 21 35 85 61 53 18 37 16 14 99 46 62 47 64 57 33
    Card 157: 24 85  5 76 32 43 58 27 28 78 | 50 36 66 65  1  2 87 75 96 18 53 39  3 48 23 97 63 90  4 62 42 93 47 54 60
    Card 158: 92 31 19 15 16 21 50 20 17 59 | 56 32 60 65 42 46  3 82 89 96 28 41 45 51 23 17 40 95 62 93 61 70 35  2 67
    Card 159: 72 26 19 38 65 12 97 76  7 34 |  4  1 45 39 60 32 22  5 28 98 43 66  2  9 21 10 15 67 78 27 90 17 68 31 35
    Card 160: 60 24 21 33 74 59 10 82 22 87 | 26 96 29 12 55 38 71 76 25 68 78  6 50 13 51 99 65 17 98 42 73 54 44 37  9
    Card 161: 82  9 94 58 56 85 45 17 21 32 | 91  1 55 63 21 94 32 22 15 17 34 85 26  9 46  7  8  5 48 82 93 60 28 56 45
    Card 162: 28 45 14 62 60 73 92 49 11 95 | 73 44 28 42 22 98 80 86 74 51 14 18 62  8 92 96 45 13 65 49 95 39 60 11 85
    Card 163: 67 45 89 16 36  9 92 91 78 43 | 75 34 52 90 85 77 27  1 83 93 32 99 69  2 79 94 80 11 35 64 39 72 17 70 13
    Card 164: 95 30 90 87 96 81 16 91 66 13 | 97 12 72 54 67  3 61 27 62 68 19  7 65 50 35 39 73 53 18 94 59  4 71 64 83
    Card 165: 56 77  8 11 88 80 65  9 78 48 | 10 26 24 21 39  5 32 80 79 48 58  9 16 69  8 30 97 53 20 78  6 56  2 88 43
    Card 166: 21 57 49 50 24 79 44  7 84 40 | 50 16 11 93 36 66 46 24 59 35 99 21  5 56 89 57 92 41 82 37 40 12 79 20 44
    Card 167: 23  2 68 93 19 28 11 22  7 27 | 11 85 94 28 22 68  2 65 21 78 92 27 93 74 38 57 83 80  7 23 19 77 90 59 84
    Card 168: 93 39 75 95 97 10 83 35 66 62 |  7 79 76 91 39 83 75 93 56 89 37 10 15 72 35 84 14 26 97 71 57 95 62 66 44
    Card 169: 95 68 15 39 27 24 62 86 71 78 | 24 27 81 70  7 48 43 86 52 88 62 58 19 68 31  8 71 80 39 78 64 95 35 15  6
    Card 170: 63  4 96 68 10  3 37 44 70 78 | 13 58 97 36 57 68 78  8 79 96 70  1 64 14 63 51 99 37 41 49  7  3 72  4 71
    Card 171: 67 36 24 10  7 11 65 28  3 83 | 87 66 58 53 21 39 40 12  1 94 61 89 22 50 84 78 72 64 92 34 71 90 11 16 15
    Card 172: 30 72 95 64  5 51 61  6 39 94 | 31 37 30 71 91 75 10 74 81 36 14 19 73 42 82 97  6 47 51 64 72 45 53 80 60
    Card 173: 64 71 93 39 91 57 82 95 60 67 | 34 97 89 36 71 78 76  7  8 33 50 68 19 73 60 91 39 82 46 67 57 35 27 10 63
    Card 174: 70 68 35 62 73 42 26  3 86 51 |  3 10 69 84 60 58 27 44 41  9 22 49 89 54 86 70  1 50 83 42 37 74 85 78 71
    Card 175:  5 82 68 57 10 32 70 72 23 45 | 10 73 36 50 23 83 29 57 34 30 61 70 59 69 45 53 12 75 86 63  6 84 64 14 97
    Card 176: 35 27 24 75 70 12 29 78 17 91 | 64 57 75  5 81 79 98 99 65 18 87 66 68 12 19 74 55 80 50 10 78 56 36 61 41
    Card 177: 82  4 32 70 79  1 29 38 87 67 | 63 15 49 46 19  8 38 69 83 75 66 87 68 95 81 54 34 82  4 64 41 73 88 11 47
    Card 178: 27  6 50 58 37 87 96 56 25 85 | 96 27 64 60 15 54 12 47 30  5 78 61 90 37 40 93 87 23 53 85 70 34 16 50 63
    Card 179: 16 29 86 94 32 82 80 71 20 38 | 52  7  9 73 14 51 27 55 99 23 80 20 49 15  6 91 47 58 26 98 63 18 77 85 94
    Card 180: 87 33 35 88 32 23 13  4 52 92 | 45 78 30 19 54 17 56 31 88 86 18 69 16 12 96 82 60 13 62 81 67 46 72 80  4
    Card 181:  7 76 43 75 70 83 39 17 92 64 | 44 91 51 57 26 90 31 63 12 70 45 38  6 35 23 17 27 79 20 59  9 66 96 62 37
    Card 182: 42 10 44 12 46 55 60 83 94 18 | 21 78 38 96 64 92 11 48 98 30 63  1 75 58 31 29 67 79 43 91 34 25 39 99  6
    Card 183: 47 21 74 24 75 48 38 33 61 39 | 15  2 21 20 95 93 54 32 77 89 81  8 16 23 30 14 58 94 35 37 50 11 83 86  4
    Card 184: 68 41 47 24 74 25 38 12  6 58 | 46 48 20 31 37 40 59  5 73 52 21  7 57 55 86 17 49 80 10 22 81  4 13 91 26
    Card 185: 76 26 24 14 90 54 75 51 13 42 | 14 30 57 20 73 56 86 42 89 74 97 37 64 51 47 24 60 83 22 87 76 90 13 61 40
    Card 186:  2 63 80 91 41 12 85 88 34 30 | 81 29 19 64 52 36 41 18 42 54 44 43 40 63 76 23 98  4 65 38 24 30 62 99 87
    Card 187: 49 77 68 73 76 60 15 92 34 39 | 32 23 47 26 40 41 43 92 84 73 94 60 24 30 45  7 14 80 76 10  5  9 34 11 50
    Card 188: 11 96 50  5 37 58 42 81 79  3 | 29 78 77 30 92 89 18 88 15 95 91 40 60 14  8 62 28 90 33 32 98 63 21 36 46
    Card 189: 78  8 49 19  1 67 44 52 54 46 | 29 64 10 55  8 78 36 67 32 91 97 80 49  1 44 83 58 46 69 34 35 94  2 14 86
    Card 190:  4 86 43 89 60 80 18 67 11 87 | 19 13 60  4  6 18 11 56 89 26 43 65 85 41 22 86 87 91 71 28 67 80  2 74 47
    Card 191: 93  9 61 81 20 51 42 14 74 83 | 68 82 91 74 80 61 44 96 18 11  9 14 39 84 20 47 17 93 42 52 99 21 51 77 76
    Card 192: 41 83 44 12 68 56 79 70 55 34 | 90  4 34 20 51 83 74 32 49 41 86 36 66 42 29 85 37 33  1 24 10 97 40 27 12
    Card 193: 63 32 91 93 31  2 42 29 30  5 | 46 56 12 16 61 41 81 20 31 89 62 93  5 58  6 73 66 55 74 29 83 88 52 47 27
    Card 194: 34 54 16  3 62  6 67 40 57 53 | 91 63 31 59 52 56  6 33 84 55  3 72 40 14 54 35 43 67 97 64 53 44 28 37 81
    Card 195:  1 18 74 57 38 11 64 30  9 54 | 82 35 66 40 29 45 14 37 19 85 54 63 99 89  5 22 71 94 11 70 39 16 42 51  8
    Card 196: 95 44 35 80 30 64 82 60 75 47 | 88 45 15 26 90 34 93 85 43 66 77 11 17 65 39 33 56 23  3 20 35  5 44 47 36
    Card 197: 53 10 14 12 65 83 18 28 79 25 |  7 37 96 80 85 51 47 39 89 13 99 72  1 21 71 25 83 61 35 59  3 73 84 50 15
    Card 198: 29 97 30 81 41 34  9 47 21 39 |  8 88 94 67 16 68 37 82 85  3 36 32 41 90 89 40 46 74 29 81  4 93 76 55 87
    Card 199: 47 48 39 87 83 25 72 74 40 29 |  1 45 19 77 47 24 20 70 85 34 62 76 91 60 16 30 35 46 21 44  3 97 65  7  6
    Card 200: 43 45 85 27 99 88 52 35 28  3 | 18 54 53 96 80 62 49 15 90 14 44 48 36 33 60 79 63 17  5 13 82 24 10 20 64
    Card 201: 71 92 68 45 33 17 99 32 96 93 | 90 82 79 26 20 85 94 61 31 84 73 30  4 87 29 28 81 27 75 39 36 58 97 98 21";

    let lines = input.split("\n");

    let mut total = 0;

    for line in lines {
        let h: Vec<&str> = line.split(":").collect();
        let h2 = h[1].replace("  ", " ");
        let numbers: Vec<&str> = h2.trim().split("|").collect();
        let winning_numbers_str: Vec<&str> = numbers[0].trim().split(" ").collect();
        let winning_numbers: HashSet<i32> = HashSet::from_iter(winning_numbers_str.iter().map(| n | n.trim().parse::<i32>().unwrap()));
        let numbers_str: Vec<&str> = numbers[1].trim().split(" ").collect();
        let numbers = numbers_str.iter().map(| n | n.trim().parse::<i32>().unwrap());

        let mut inner_total = 0;
        for number in numbers {
            if winning_numbers.contains(&number) {
                if inner_total == 0 {
                    inner_total = 1;
                } else {
                    inner_total = inner_total * 2;
                }
            }
        }

        total += inner_total;
    }

    println!("{}", total);
}

fn day_four_two() {

    let input = "Card   1: 30 48 49 69  1 86 94 68 12 85 | 86 57 89  8 81 85 82 68  1 22 90  2 74 12 30 45 69 92 62  4 94 48 47 64 49
    Card   2: 57 32 92 73 76 62 11 19 61 90 | 19 82 53 87 57 80 69 76 90 56 11 61 30 92 73 99  4 32 33 64 34 62 27 78 65
    Card   3:  6 56 40  1 47 26 25 87  4  2 | 12 26 32 25  8  4 41 54 69 99  2 45 70  6 59 23 47  7 49 17  1 56 92 87 40
    Card   4: 26 49 44 82 25 43 47 74 97 13 | 76 62 13 82 55 26 93 84 83 19 47 22 49 44 25 43  7 18  9 45 97 15 90 85 74
    Card   5: 88 58 96 80 56 16 55 13  3 40 | 20 57 23 71 76 43 36 72 52 18 60 28 80 84 64 75 93 46 19 69 25 31 58 45 47
    Card   6: 89 88 34 62 60 41 15 42 57 58 | 58 49 82 42 70 78 72 57 77 47 62 89 30 60 96 98 54 66 25 14  6 34 15 41 88
    Card   7: 97 73 29 26 91 16 66 64 70 23 |  3 12 61 11 70 59 71  1 26 44 14 64 63 93 20 17 89 16 43 96 52 92 46 97 85
    Card   8:  9 33 46 87 34 97 71 30  5 43 | 21 64  8 43 77  9 20 46 10 19 53 33 71 56 39 87 34 50 72 91 30 89 13  5 97
    Card   9: 72 41 12 58  8 56 11 82 22 66 | 27 97 48 14 77 37 33 91 85 39 75 42 58  2 52  9 70 45 72 62  5 21 23 60 99
    Card  10:  3 46 56  2 58  4 92 80 86 52 | 15 56 91 57 61  8 87 34 51  3 23 41 96 45 48 49 43 25 26  9 58 50 52 38 12
    Card  11: 76 94  4 55 97  8 22 99 80 70 | 64 75 23 49 42  1 66 54 85 29 28 94 22 93  4 69 72 62 41  2 86 83 97 34 82
    Card  12: 65 91  7 24 80 20 46 29 58 57 | 51 59 52 63 28 44 62 35 46 49 24 14 58 91 65 78 84 20 43 32 23 15 29 13 22
    Card  13: 98 94 55 43 83 72 19 46 45 22 | 55 67 59 91 46 37 24 52 98  4 72 22 44 94 88 43 27  7 75 13 45 82 15 65 19
    Card  14: 92 51 81  6  5 91 73  9 80 94 | 56 72 71 32 92 98 15  2 10 91 62  1 51 74 69 89 33 96 39 97 21 86 65 52 83
    Card  15: 84 40 87 91  3 61 20 73  2 37 | 77  3 37 81 20  9 51  2 87 73 53 66 26 36 69 99 83 96 88  5 91 18 40 57 80
    Card  16: 97 63 52 67 86 87 91 25 69 43 | 68 67 75  3 85 10 88 99 52 43 95 92 80 87 11 13 96 58  1 17 74 20 36 93 77
    Card  17: 28 90 77 78 52  7 94 13  6 88 | 11 69 49 56 59 14  7 24 53 21 67  2 17  8 47 66 48 30 79 43 86 64 46 28 23
    Card  18: 50 22 25 69 54  4 37 14 49 75 | 82 84 78 88 87 15 80  1 11 91 89 93 25 98 74 42  6 59 23 58 13 46 49 32 95
    Card  19: 96 56 86 38 29 43 69 66 26 46 | 80 13 90 30 41 86 44 95 71 23 40 75 33 14 56 59 89 18 92 79 15  4 84 68 82
    Card  20: 95 26 31 69 63 25 56 87 74 48 | 88 16 42 22 30 14 71 62 35 87 99  3 96 54 41 83 40 29 79 20 50 24 53 10 66
    Card  21: 75 76 94 26 24 56 34 35 93 87 | 42 23 71 55 44 79 12  4 78 32 82 53 40 35 43 50  9 65 29 57 93 86  7 67 19
    Card  22: 53 58 25 33 42 98 94 95 97 22 | 44 91 40  1 88 31 61  6 54 52 47 93  3 99 27 59 15 53 56 39 66 43 51 41 48
    Card  23: 34 95 33 45 56 37 30 94 76 67 |  9  1 72 68 51 66 36 24 21 92 35 50 62 86 90 49 82 46 15  4 70 40 75 64 73
    Card  24: 73 93 49 22 86 18 31 87  3  2 | 27 42 46 97 49 31 47 25 93 74  9 60 44 87 15 63 18  2 73 89  3 62 86 65 22
    Card  25: 36 24 95 21 31  7 56 93 82 17 | 82 36 23 54 66 83 95 31 65 16 17 86  7 21 61 24 55 56 13 53 59 64 96 39 32
    Card  26: 16 89 92 12 40 17  3 80 20 21 | 37 72 26  9 79 55 74  2 92 59 17 35 71 94 98 31 58  3 32 51 21 88 40 96 57
    Card  27: 70 19 81 48 62 26 45 86 65 41 | 63 38 71 43 50 42 25 14 18 54 66 96 29 36 95 73 20 76 16  6 58 78 56 87 91
    Card  28: 61 66 57 75 52 20 41 21 24 14 | 83 51 77 14 66 91 75  3 47 20 18 65 76 52  5 11 72 57 10 36 30 92 73 28 19
    Card  29: 94 39 68 79 28 70 81 97 59 48 | 46  4 41 39 38 75 18 76 45 31 79 97 91 64 71 85 63 55  8 94 67 59 36 74 49
    Card  30: 90 45 15 47  1 24 99 62 69 55 | 42  5  1 37 77 71 66 57 94 18 26 47 46 27 48 80 24 95 30 15 53 90 38 99 69
    Card  31: 53 16 59 23 78 95 38 17 40  7 | 86 20  7 98 44 19 73 36 40  4 58 16 94  2 83 32  6 78 57 13 35 30 87 51  5
    Card  32: 64 79 78  6 24 15 44 45 82 33 | 34 18 80 94 54 59 14 23 77 25 73 81 26 62 17 45 37 82 67 24 98 78 85 43 13
    Card  33: 91 97 83 95 89 32 86 94 96 19 | 93 39 57 58 41 62  1 80 33  2 24 51 27 25  6 10 78 76 63 65 38 79 61 98 90
    Card  34: 41 84 69 81 95 71  7 80 65 55 | 88 17 73  3 54 66 68  8 76 84 36 79 55 47 58 91 37 53 57 46 93 71 83 42 12
    Card  35: 51 67 16 48 34 86 59 74 98 65 | 14 75 23 98 48 83 11 35 18 93 68 50 85 31 40 82 27 92 55 63 64 37 76  2 33
    Card  36: 26 80 18 67 87 93 79  1 59 61 | 83 51 97 34 70 90 49 44 39 95 92 38 55  9 65 96 94 82 61  1 20 46 50 63  6
    Card  37: 71 13 91 66 24  3  1 41 27 75 | 12 38 45  5 80 11  8 59 90 64 85 49 52 56 35 96  7 20 53 39 44 22 76 57 66
    Card  38: 21 56  2 42 51 28 37 38 95 12 |  3 84 55 81 91  6 90 14 68 93 64 33 78  7  4 60 18 54 11 29 26 19 75 72 52
    Card  39:  1 40 97 60 90 47 74 41  7 58 | 43  8 47 72  7 78 41 96  5 37 98 97 58 89 23 65 74  1 92 90 28 16 60 40 67
    Card  40: 75 99 14 36 64 42 30  8 52 97 | 64 89 36 80 99 30 57 40 35 75 53 97 52 38 68  1 74 42 14 60 49 79  8 72 20
    Card  41: 26 38 87 47 39 62 56  2 49 51 | 25 45 87 63 24  2 39 51 54 56 70 62 77 57 68 49 31 34 26 72 42 38 47 94 29
    Card  42: 26 69 10 47 89 27 43 62 97 50 | 62 79 50 69  2 97 68 87 27  5 67 18 64 26 10 25 91  4  7 43 70 14  1 54 89
    Card  43: 92 16 95 21 15 77 27 88 76 63 | 92 77 78  1 58 96  8 31 63 16 14  2 54 88 15 52 73 76 21 27 84 71 25 95 72
    Card  44: 27 31 64 16 96 95 74 57 59 53 | 75 20 48 11 18 44  3 45 79 98 12 54 30 60 32 15 13 72 21 87 85 52 70 33 35
    Card  45: 62 31 89 91 85 83 93 11 26 23 | 53 55 71 62 14 65  8 21 96 57 17 91 51 82 86 47 64 80  3 69 56 90 85 32 70
    Card  46: 52 67 93 77 39 13 55 63 68 78 | 19 96 76 50 11 61 72 81 64 10 84 22 83 37  3 58 80 48 57 18 59 62 92  4  9
    Card  47: 31 14 25 56 83 48 53 64 39 77 | 48 34 27 16  5 55 77 42 17  6 24 49 37 92 51  8 97 52 41 11 31 80 47 87 38
    Card  48: 69 65 89 40  1 31 64 50 48 38 | 44 75 74 58 13 56 50 80 22 89 47 95  8 53 31 38 48 64 72 39  1 52 43 17 65
    Card  49: 30 49 86 87 78 96 73 44 21 90 | 14 39 75 49 96 63 56 73 64 81 93 98 99  1 13 65 54 88 61 83 44 30 45 15 67
    Card  50: 61 79 65 67 77 58 21 37 90 48 | 81 79 62 51 21 85 61 90 92 47 29 23 80  7 98 40 59 87 18 77 48  6 99 88  1
    Card  51:  9 45 35 38 92 47 27 61 91 67 | 75 86 50 74 41 92 59 71 52 91 97 73 96  4 19 87 69 76 98 21 90 37 32 14 77
    Card  52: 82 80 79 33 18 70 52 73 38 39 | 21 82 95 25 66 89 75 85 84 39 47 28 49 45 79 17 56 10 29 52 44 77 70 32 65
    Card  53:  2 47 76 33 17 62 13 70  9 31 | 84  2 31 47 13 97  4  3 33 21 17  8 62 10 19 78 41 70 43 98 79 59 95 29 99
    Card  54: 50 10 89 77 41 82 43 84 30  7 | 28 49 35 76  9 23 65 71 60 36 32 15  3 61 95 10 24 77 17 97 48 62 88 51 68
    Card  55: 98 86  8 43 54 59 11  7 63 42 | 24 98  1 58 51 68  2 56  8 54 29 25 66 17 59 86 30 48 43 60  5 55 18 44 28
    Card  56: 88 38  8 43 91 12 69 45 71 20 | 11 34  1 95 77 19 39 40 33  5 80 51 75 94 84 23 82  8 86 62 74 89 96 22 66
    Card  57: 95 38 81 99 57 68 60 55 39 18 | 34 38 45  7 63 12 97 35 44 17 57 58 92 41 89 30 39 47 66 10 82 52 68 55 98
    Card  58: 34 72 18 15 20  1 28 89  7 35 | 14 22 61 36 84 26 55 62 32 53 44 51 46 17  6 54 85 35 95 40 10 92 42 19 83
    Card  59: 58 43 86  3 73 17 24 49 29  5 | 87 93 56 60 35 48 86  8  7 16 67 89 73 43 69 31 88 23 50 59 45 84 61 26 65
    Card  60:  4 74 59 28 17 23 67 48 82 20 | 76 91 88 47 84  2 14 61 56 11 58 34 24 99 75 66 33 96 92 94 64 73 72 45  7
    Card  61: 24 40  2 95 58 17 28 43  4  8 | 97 77 49 15 51 41 70 80  6 74 11 68 38  5  8 64 81 30 89 72 63 52 39  7 82
    Card  62: 79 22 90 64 37 95  4 87 30 38 | 73 25 91 13 83 45 77 53 88 63 43 92 78 34 32 58 24 40 50 61 55 41  2 67 14
    Card  63: 70 13 43 35 58 84 44 51 65 73 | 43  3 48 51 69 70 31 98 19 15 91 61 55 39 41 58 44 65 35 13 25 73  9 50 56
    Card  64: 85 64 44 45 31 95  3 16 19 74 | 53 13  9 89 83 58 47 90 77 59 98 99 74 91 52 15 61 80 75 88 54 23 24  8 29
    Card  65: 70 33 34 72 80 56  7 95 71 32 | 16 32 56 34 73 31 45 29 71 33 98 72 97 19 39 94 91 90 70 75 40  7 95 80 96
    Card  66: 29 30 35 39 52 17 25  1 62 79 | 83  2 69 13 36 76 59 50 90 64 30 37 48 75 95 25 26 60  7 29 86 42  8 22 11
    Card  67: 51 59 72 52 60 74 20 68 90 40 | 51 34  8  2 67 46 47 10  9 43 61 56 28 74 73 60 81 25 87  4 17 84 65 42 32
    Card  68: 36  3 98 39  7 74 66 85 81 14 |  7 78 14 74 85 97 55 62 66 98 28 94 39 81 42  6 24 36 60 50 75 87  3  9 18
    Card  69: 72 99 53 87 83 54 58 32 79 26 | 54 70  5 34  1 79 26 99 90 11 77 13 98 83 15 58 40 96 91 12 32 53 87 72 43
    Card  70: 72 26 89 44  9  7 14 95 46 51 | 45 44 91  3 71 77 82 56  2  6 24 20 49 75 60 22 58 80 42 68 32 54 76 29 79
    Card  71: 56 75 58 97  4 95 16 23  7 71 | 63 96 40 64 42 38 65 48 78 35  5 24 20 41 70 89 45 17 49 21 10 44 92 60 51
    Card  72: 82  3 37 72 19 78 84 69 43 62 | 95 43 97 82 72 14 70 47 45 26 92 77 67 62 49 22 19  3 37  7 69 78 94 84  6
    Card  73: 63 77 50 12 46 80 13 54 64 24 | 60 50 17 87  1 46 42 13 80 63 82 48 45 35 55 29 30 31  9 24 64 27 77 40 69
    Card  74: 66 73 38  5 89  1 11 10 91 92 | 89 23 28 50 82 63 33 91 72 74 73 66 24 17 42 11 36 94 79 92  4 85 29 57 10
    Card  75: 66 51 34 46 78 27 89 42 52  4 | 87 79 28 18  1 35 81 60 65 76 56 78 42 43 47 45 89 70  4 68 19 63 33 94 86
    Card  76: 35 43 54 87 15 71 30 92 29 24 |  6 76 36 34 50 47 88 29 44 73 68 97 87 24 81 20 94 86 70 90 71  2 15 98 54
    Card  77: 27 77 81 31 53 47 45 71 73 41 | 85 40 63 68 22 10 75 55 62 67 54 49 51 79 17 92 93 21 73 38 37 74 90 23 98
    Card  78: 79  1 93 23 31 48 32 53 64 57 | 52 76 35 68  3 75 22 81 79 51 80 46 88 65 83 44 67 89 31 86  8 98 97 16 30
    Card  79: 72 80  7 74 10 22 12 34 89 97 | 47 62  5 23 17 27  9 21 78 68 66 98 29 39 51 83 24 90 64 10 48 22 97  2 35
    Card  80: 80 94 30 12 28 54 78 34 58 63 | 46 33 86 44 50 94 96 66 57 58 87 65 92 71 32 56 73 90 11 85 21 76 45 27 64
    Card  81: 94 60 72 30 14  3 40 86 69 82 | 74 99 11 55 27 54 70 46 89 24 44 85  6 53 58 15 26 10 20 38 56 63  1  4 12
    Card  82: 88 30 84 71 61 63 20 56 49 89 | 10 87 14  2  9 57 46 27 12 41 78 59  5 55 19 94  7 24  1 25 13 28 42 33 68
    Card  83:  6 35 37 64  9 44 21 42 56 30 | 57 69 83 89 47 74 95 71 81 84 41 45 40  2  1 77 32 75 90 60 62 82 79 22 55
    Card  84: 34 13  4 21 88 26 18 96 38  3 | 16 66 63  9  7 95 28 72 74 41 10 85 30 70 91 69 47 44 33 49 60 46 57 62 36
    Card  85: 94 58 24 74 61  5 49 99 30 54 | 99 11 61 58 25  4  2 55 30 75 62 51 94 22  3 36 54 84 71  9  8 24 70 90  5
    Card  86: 33 82 19 66 52 24 76 34  5 84 | 82 37 63 83 36 22 13 76 84 19 38  6 65 33 66 40 95 52 24 45  5 88 34 89 49
    Card  87: 86 24 92  7 72 28 63 23 12 82 |  9 59 36 56 40  1 76  4 88 10 85 14 17 15 61 29 57 50 98 49  3 41 42 93 22
    Card  88: 23 32 78 34 12  8 89 20 50 29 | 59 24 13 72 14  7 83 37 32 39 90 46 96 49 94 82 43 33 85 92 69 62 57 48  9
    Card  89: 98 48 90 71 56 21 61 86 29 63 | 50 62  2 23 70 44 36 35 57 48 13 64 21 90 93  1 53  5 33  7 29 67 38 61 54
    Card  90: 27 66 35 40 54 68  1 80 42 49 | 97 94  2  1 66 55 15 49  5 75 26 30 20 25 73 79 10 40 57 85 23 43 68 38 14
    Card  91: 52 74 25 22 84 68 76 27 89 13 | 52 75 90 22 73 94 64 72 84 70 74 67 30  2  1 29 54 86 25 58 41  6 43 91 34
    Card  92: 35 83 75 46 37 28 82 54 13 77 | 96 77 46 33 37 59 62 13 28 54 75 86 74 35 47 85  6 80 55 20 36 83 38 82 65
    Card  93:  8 19 27 84 17 42 23 22 78 16 | 76 16 63 78 89  1 27 57 83 34 36 69 42 95 41 44 74 28 79 39  4 92 19 23  7
    Card  94: 90 53 59 26 63 91 50 81 69 85 | 20 78 12 23 31 91 42 36 75 95 19 76 96 49 94 47 63 53 27 88 67 69 85 60  7
    Card  95: 87 25 89 31 63 32 13 80 60 14 | 95 82 78 34 87 13  3 60 88 28 96 59 36 77 14 79 55 35 83 25 73 58 48 63 86
    Card  96: 32 27 21 77 35  9  8 28 97 82 | 53 97 29 81 66  3 17 85  8 35 56 38 26 60 49  9 39 86 28 71 89 88 54 44 27
    Card  97:  2 42 48 43 96 55 51 60 33 89 | 85  3 31 88 98 25 41 81 44 68  4 80 63 17 45 57 89  7 38 76 95 53 49 67 13
    Card  98: 63 57 83 58 23 27 92 21  2 76 | 58 76 95 80 62 56 98 33 59 46 51 23 93 69 96 48 36 53 19  8 45 32 42  2 89
    Card  99: 28 85 31 55 69 73 70 66 11 79 | 22  4 83 55 26 20 29 72 54 12 30 99 66 14 10 17 15  8 88 45 76 69 58 89 75
    Card 100: 99 91 55 84 81 61 94 95  8 25 | 30 15 52 88 27  6 53 41 45 95 66 19 98 86 26 31 54 43 51  9 82 97 57 60 16
    Card 101: 86 47 76 89 85  1 34 35 53 51 | 81 13 69 14 79 68 37 39 63 59 50 15 56 99 92 80 36 43 18 77 65 19 45 91  8
    Card 102: 98 27 49 51 14 77  6 41 97 64 | 60 76  3 79 48 42 29 37 10 24 74 62 11 17 95 73 56 85 28 38 90 63 84 30 94
    Card 103: 75 45 64 88 40 69 96 66 52 28 | 28 12 88 45 60 64 16 96 41 17 82 97 24 75 54 67 92 14 40 85 93 52 31 37 66
    Card 104: 36 69 82 51 81 42 92 95 62 73 | 13  5 65 87 70 36 95 99  9 62 24 55 31 42 51 82  7 34 69 92 14 18 79 81 73
    Card 105: 87 81 71 62  3 41 17 86 88  5 | 98 31 80 39 25 35 47 42 95 14 91 83 65 12 87 96 88 58  5 19 56  1 97 62 48
    Card 106: 31 55 80 86  7 20 37  2 89  4 | 70 20 92 62 38 86  7 93 79  6 48 37 19 21 89 47 25 78 43 46 54 69 63  8 80
    Card 107: 99 85  5 37 34 95 11 12 51 96 | 69 64 67 53 15 81 89  3  2 54 49 98 71 72  1 76 91  8 17 51 23 83 21 86 92
    Card 108: 95 57 98 30  3 83  6 64 52 18 | 59 88 91 78 61 30 52 98 65 68 25 17 13  3 64 89 57 37 83  6 95 36 97 20 18
    Card 109: 97 99 89 96 69 84 62 66 61 83 |  9 31 22 49 96 84 17 61 15 99 39 89 26  4 92 32 97 21 62 56 66 82 69 83 13
    Card 110: 61 93 64 41 91 76 74 21 56  5 | 51 65 17 29 19 45 70 87 40 10 62 20 89 85 53 36 42 37 50  2 74 56 64 46 48
    Card 111:  8 85 35 16 52 31 94 10 29 82 | 70 60 11  6 31 58 10 52 43 35 16 30 94 62  1 29 74  9  4 90  7 59 40 20 85
    Card 112: 96 15 95 19 76 41 77 62 46 89 | 61 35 99 56 60 82 17 16 47  4 67 20 91 79 78 97 40  7 13 43 87 74 34 31 84
    Card 113: 28 83 44 97 18 22 96 49 17 98 | 86 88 11 92  5 54 90 51 38 52 36 13 64 81 45 63 85 15 47 40 18 50 67 39 73
    Card 114: 89 28 69 44 47 58 20 60 25 92 | 97 61 98  4 37 30 68 96 86  6 50 34 71 53 78 57 10 36 35 40 67 33 93 17 75
    Card 115: 20 49 40 31  8 55 28 79 99 22 | 95 87 21 14 62 17 70 81 83 54 96 39 18 35 44 61 23 30 80 89 42 50  7 58 47
    Card 116:  3 75 52 97 60 24 50 13 54 71 | 17 21  1 98  8 79 59 88 97 53 82 28 34 45 93 69 13 85 38 26  7 37 43  5 22
    Card 117: 44 70 59 87 64 20 17 19 68 71 |  6 46 47 79 35 74 78 26 19 99 20 80 31  1 68  3 12 39 59 62 94 25 91 92 88
    Card 118: 65 74 73 48 97 21  1 43 57 29 | 73 25 58 67 56 41 19 20  1 12 99 48 47 96 68 50 88 49 80 10 39 75 11 82 79
    Card 119: 68 40  2 54 25 75 77 14 50 86 | 27 69 39 90 81 15 17 54 63 51 87 52 37 22  7 38 11  8 24 96 42 60  6 99  4
    Card 120: 43 63 34 38 57 39 23 29 16 41 | 90 93  1 72  9 80 94 73 28 65 87 25 78 33 98 66 52  2 36 30 92 54 47 24 12
    Card 121: 92 44 29 97 22 41 73 16 31 65 | 20 51 50 43 30 27 39 40 88 14 57 77 62 90 18 87 26 54 95 48 34 10 93 47  9
    Card 122: 61 12 44 52  4 69 42  1 77 54 | 24 23 11 39 78 52 12 69 73 42 47 43  4 29 37  1 36 77 19 61 95 54 94 44 28
    Card 123: 17 44 12 66 30 19 96 54 34 75 | 24 17 42 90 14 92 84 35 71 12 34 32 99 19 30 98 66 70 54 44 56 96 77 36 75
    Card 124: 99 74 53 87 15 93 58 96 24 44 | 15 58 96 29 49 50 87 93 86 53 12 51 24 44 60 74 19  8 67 99 76 72 45 80 81
    Card 125: 28 95  5 13 21 49 70 33 48  7 | 61 57  3 65 89 13  7 70  1 95 84 33 46 21 28 79  5  9 19 48  4 76 82 86 49
    Card 126:  1 83 21 50 35  3 55 84 38 81 | 61 80 30 26 83 84 38 92 73 21 42 14  1  6 81 15 50 57 55 56 35 63 67  3 93
    Card 127: 37  8 99 82 24  6 62 63 94 29 | 62  8 83 82 24 87 46 55 78 91 31 23 29 16 99 63 54  9 68 48 94  5 37  6 40
    Card 128: 18 47 91 78 74 63 49 42 85 10 |  4 10 82 20 97  3 71 74 62 95 60  9 28 73 47 32 14 18 49  7 83 92 85 53 78
    Card 129: 19  9 11 30 91 83 99 93 46 22 | 38 33 91 85 57 31 15 99 97 54 17 50 71  8  6 29 59 36 75 81 82 67 44 65 25
    Card 130: 62 13 98 39 78 28 63 38 41  6 | 51 99  1 98 81 40 39 63 62 52 25 47 55 13 34 41 38 18 14 73 97 28  6 48 78
    Card 131:  9 69 43 61 65 35  4 59 71 96 | 17  4 65 29 19 48  7 32 69 61 23 15  1 37  9 80 59 47 64 41 35 79 71 43 96
    Card 132: 40 62 73 49 19 51 30 95 92 55 | 77 79 31 51 92 41 49 27 13 40 99 52 68 98 32 26 29 20 60 25 65 72 11 70 19
    Card 133:  7 73  1 68 16 40 10 37 78 91 | 39  9 33 97 98 13 35 38 72 75 82 37 27 32 93 48 87 50 94  5 95 67 46 24 54
    Card 134: 43 78 49 90 73  7 38 95 12  4 | 10 79 30 12 25 39 32 68 95 78 45 40 73 37 70 49 41 46 91 42 23 18 76 15 90
    Card 135: 83 66 13 30 47 75 52 69 17 48 | 76  6  7  1 12 64 89 62 52 59 83 97 33 28 66 20 72 13 84 65 56 73 53 92 30
    Card 136: 63 90 52  1 43 85 83 22 47 57 | 17 44 77 36 79 20  1 65 29 50  6 81 78 76 70 97 90 68 67 34 31 82 88  2 25
    Card 137: 77 25 91 14 87 80  8 90 27 57 | 91 63 29 60 71 57 98 14 52 77 39 96 85 50 76 10  8 27 32  4 23  3 17 62 24
    Card 138: 33 17 89 10 85 90 79 72 13 53 | 30 45 81 89 47 57 32 80  1 82 84 97 16 90 66  4 75 94 29 54  9 78  3  8 93
    Card 139: 57 99 35 94  5 11 86 67 34 96 |  1 79 54 17 20 82 26 32  7 77 44 51 18  3 76 64  9 72 35 83 37  5 60 66 22
    Card 140:  6  9 44 98 64 92 30 14 62  4 | 43 49  9 79 42 27 20 69 36 24 47 19 14 38 98 54 77 73 41 56  4 15 86 10 44
    Card 141: 28 99 14 96 19 92 12 23 17 22 | 64 19 85 70 80 16 63 96  2  3 61 67 41  9 74 12 37 30 86 36 26 92 94 66 71
    Card 142: 28  2 68 35 71 10 98 57 79 14 | 44 78 23 24 59 49 21 47 75  6 60 40 22 15 46 41 71 53 87 90 32 52 35 72 37
    Card 143:  8 10 60 94 66 51 36 29 56 69 | 71 69  2  9 12 55  1 53 91 84 22 83 77  6 18 57 10 24 44 72 79 73 95 62 99
    Card 144: 21 12 42 22 50 10 88 18 19  9 | 25 72 96 47 60 85 23 10  7  3 26 81 59 80 38 91  5 70 95 97 53 33 67 74 92
    Card 145: 53  5 14 26 75 35 21 39 30 43 | 13 18  6 46 69 76 78 67 81 68 36 83 91 97 32 40 23 92 29 28 42 51 71 20 37
    Card 146: 18 66 74 11 31 91 15 55 95 87 | 54 74 55 18 89 28 95 72 62 38 15 92 91 31 44  9 81 87 88 66 24 23 71 16 11
    Card 147:  7 21 81 38 56  2 55 50 60 83 | 15  9 91 69 44 92 54 99 72 43 59 78 25 79 32 51 64 20 62 24 75 38 29 16 97
    Card 148: 46 52 47 60 29  9 67 22 10 23 | 92 82 54 91 74 55  1 73 26 61 64 90 28  2 72 40 27 32 51 20 89 31 75 97 99
    Card 149: 27 58 78 77 96 68 54 89 31 86 | 48 42 86  8 21 20 60 76 72 37  2 75 33 67 57 28 41 78  1 49 95 87 38 94 62
    Card 150: 38 19 90 28 21 59 10  7 66 80 | 68 60 40  3 50 55  6 65 64 59 47 20 56 70 14 29 26  4 22 74 17 69 46 75 87
    Card 151: 93 89 25 22 91 70 10 78 84 50 | 21 35 78 58 30 71 11 70 99 84 97 87 57 67 15  9 20 91 79 50 26 74 47 56 12
    Card 152: 68 34 95 27 26 87 56 85 92 78 | 58 33 87 81 64 95 27 48 85 72  6 62 65 50 39 96 37 92 28 80 60  1 83 31 74
    Card 153: 34 11 38 50 57 78 52 73  7  4 | 56 78  9 11 32 92 26 86 40 27 24 91 18 13  4 77 37 61 25 58 60  8 30 16 80
    Card 154: 40 88 81 92  6 33 60 25 98 37 | 77 28 68 92 73 41 81 24 40 67 11 44 17 91 82 39 69  8 57 62 29 76 99 47 79
    Card 155: 15 95 17 21 76 78 69 54  1 40 | 68 17 89 96 23 49 20 53 47 93 37 55 10 69 21  4  3  6 75 65 95 79 67 30 91
    Card 156: 82 50  7 62 47 55 85 26 41 17 | 71 94  8 43 83 90 76 20 66 21 35 85 61 53 18 37 16 14 99 46 62 47 64 57 33
    Card 157: 24 85  5 76 32 43 58 27 28 78 | 50 36 66 65  1  2 87 75 96 18 53 39  3 48 23 97 63 90  4 62 42 93 47 54 60
    Card 158: 92 31 19 15 16 21 50 20 17 59 | 56 32 60 65 42 46  3 82 89 96 28 41 45 51 23 17 40 95 62 93 61 70 35  2 67
    Card 159: 72 26 19 38 65 12 97 76  7 34 |  4  1 45 39 60 32 22  5 28 98 43 66  2  9 21 10 15 67 78 27 90 17 68 31 35
    Card 160: 60 24 21 33 74 59 10 82 22 87 | 26 96 29 12 55 38 71 76 25 68 78  6 50 13 51 99 65 17 98 42 73 54 44 37  9
    Card 161: 82  9 94 58 56 85 45 17 21 32 | 91  1 55 63 21 94 32 22 15 17 34 85 26  9 46  7  8  5 48 82 93 60 28 56 45
    Card 162: 28 45 14 62 60 73 92 49 11 95 | 73 44 28 42 22 98 80 86 74 51 14 18 62  8 92 96 45 13 65 49 95 39 60 11 85
    Card 163: 67 45 89 16 36  9 92 91 78 43 | 75 34 52 90 85 77 27  1 83 93 32 99 69  2 79 94 80 11 35 64 39 72 17 70 13
    Card 164: 95 30 90 87 96 81 16 91 66 13 | 97 12 72 54 67  3 61 27 62 68 19  7 65 50 35 39 73 53 18 94 59  4 71 64 83
    Card 165: 56 77  8 11 88 80 65  9 78 48 | 10 26 24 21 39  5 32 80 79 48 58  9 16 69  8 30 97 53 20 78  6 56  2 88 43
    Card 166: 21 57 49 50 24 79 44  7 84 40 | 50 16 11 93 36 66 46 24 59 35 99 21  5 56 89 57 92 41 82 37 40 12 79 20 44
    Card 167: 23  2 68 93 19 28 11 22  7 27 | 11 85 94 28 22 68  2 65 21 78 92 27 93 74 38 57 83 80  7 23 19 77 90 59 84
    Card 168: 93 39 75 95 97 10 83 35 66 62 |  7 79 76 91 39 83 75 93 56 89 37 10 15 72 35 84 14 26 97 71 57 95 62 66 44
    Card 169: 95 68 15 39 27 24 62 86 71 78 | 24 27 81 70  7 48 43 86 52 88 62 58 19 68 31  8 71 80 39 78 64 95 35 15  6
    Card 170: 63  4 96 68 10  3 37 44 70 78 | 13 58 97 36 57 68 78  8 79 96 70  1 64 14 63 51 99 37 41 49  7  3 72  4 71
    Card 171: 67 36 24 10  7 11 65 28  3 83 | 87 66 58 53 21 39 40 12  1 94 61 89 22 50 84 78 72 64 92 34 71 90 11 16 15
    Card 172: 30 72 95 64  5 51 61  6 39 94 | 31 37 30 71 91 75 10 74 81 36 14 19 73 42 82 97  6 47 51 64 72 45 53 80 60
    Card 173: 64 71 93 39 91 57 82 95 60 67 | 34 97 89 36 71 78 76  7  8 33 50 68 19 73 60 91 39 82 46 67 57 35 27 10 63
    Card 174: 70 68 35 62 73 42 26  3 86 51 |  3 10 69 84 60 58 27 44 41  9 22 49 89 54 86 70  1 50 83 42 37 74 85 78 71
    Card 175:  5 82 68 57 10 32 70 72 23 45 | 10 73 36 50 23 83 29 57 34 30 61 70 59 69 45 53 12 75 86 63  6 84 64 14 97
    Card 176: 35 27 24 75 70 12 29 78 17 91 | 64 57 75  5 81 79 98 99 65 18 87 66 68 12 19 74 55 80 50 10 78 56 36 61 41
    Card 177: 82  4 32 70 79  1 29 38 87 67 | 63 15 49 46 19  8 38 69 83 75 66 87 68 95 81 54 34 82  4 64 41 73 88 11 47
    Card 178: 27  6 50 58 37 87 96 56 25 85 | 96 27 64 60 15 54 12 47 30  5 78 61 90 37 40 93 87 23 53 85 70 34 16 50 63
    Card 179: 16 29 86 94 32 82 80 71 20 38 | 52  7  9 73 14 51 27 55 99 23 80 20 49 15  6 91 47 58 26 98 63 18 77 85 94
    Card 180: 87 33 35 88 32 23 13  4 52 92 | 45 78 30 19 54 17 56 31 88 86 18 69 16 12 96 82 60 13 62 81 67 46 72 80  4
    Card 181:  7 76 43 75 70 83 39 17 92 64 | 44 91 51 57 26 90 31 63 12 70 45 38  6 35 23 17 27 79 20 59  9 66 96 62 37
    Card 182: 42 10 44 12 46 55 60 83 94 18 | 21 78 38 96 64 92 11 48 98 30 63  1 75 58 31 29 67 79 43 91 34 25 39 99  6
    Card 183: 47 21 74 24 75 48 38 33 61 39 | 15  2 21 20 95 93 54 32 77 89 81  8 16 23 30 14 58 94 35 37 50 11 83 86  4
    Card 184: 68 41 47 24 74 25 38 12  6 58 | 46 48 20 31 37 40 59  5 73 52 21  7 57 55 86 17 49 80 10 22 81  4 13 91 26
    Card 185: 76 26 24 14 90 54 75 51 13 42 | 14 30 57 20 73 56 86 42 89 74 97 37 64 51 47 24 60 83 22 87 76 90 13 61 40
    Card 186:  2 63 80 91 41 12 85 88 34 30 | 81 29 19 64 52 36 41 18 42 54 44 43 40 63 76 23 98  4 65 38 24 30 62 99 87
    Card 187: 49 77 68 73 76 60 15 92 34 39 | 32 23 47 26 40 41 43 92 84 73 94 60 24 30 45  7 14 80 76 10  5  9 34 11 50
    Card 188: 11 96 50  5 37 58 42 81 79  3 | 29 78 77 30 92 89 18 88 15 95 91 40 60 14  8 62 28 90 33 32 98 63 21 36 46
    Card 189: 78  8 49 19  1 67 44 52 54 46 | 29 64 10 55  8 78 36 67 32 91 97 80 49  1 44 83 58 46 69 34 35 94  2 14 86
    Card 190:  4 86 43 89 60 80 18 67 11 87 | 19 13 60  4  6 18 11 56 89 26 43 65 85 41 22 86 87 91 71 28 67 80  2 74 47
    Card 191: 93  9 61 81 20 51 42 14 74 83 | 68 82 91 74 80 61 44 96 18 11  9 14 39 84 20 47 17 93 42 52 99 21 51 77 76
    Card 192: 41 83 44 12 68 56 79 70 55 34 | 90  4 34 20 51 83 74 32 49 41 86 36 66 42 29 85 37 33  1 24 10 97 40 27 12
    Card 193: 63 32 91 93 31  2 42 29 30  5 | 46 56 12 16 61 41 81 20 31 89 62 93  5 58  6 73 66 55 74 29 83 88 52 47 27
    Card 194: 34 54 16  3 62  6 67 40 57 53 | 91 63 31 59 52 56  6 33 84 55  3 72 40 14 54 35 43 67 97 64 53 44 28 37 81
    Card 195:  1 18 74 57 38 11 64 30  9 54 | 82 35 66 40 29 45 14 37 19 85 54 63 99 89  5 22 71 94 11 70 39 16 42 51  8
    Card 196: 95 44 35 80 30 64 82 60 75 47 | 88 45 15 26 90 34 93 85 43 66 77 11 17 65 39 33 56 23  3 20 35  5 44 47 36
    Card 197: 53 10 14 12 65 83 18 28 79 25 |  7 37 96 80 85 51 47 39 89 13 99 72  1 21 71 25 83 61 35 59  3 73 84 50 15
    Card 198: 29 97 30 81 41 34  9 47 21 39 |  8 88 94 67 16 68 37 82 85  3 36 32 41 90 89 40 46 74 29 81  4 93 76 55 87
    Card 199: 47 48 39 87 83 25 72 74 40 29 |  1 45 19 77 47 24 20 70 85 34 62 76 91 60 16 30 35 46 21 44  3 97 65  7  6
    Card 200: 43 45 85 27 99 88 52 35 28  3 | 18 54 53 96 80 62 49 15 90 14 44 48 36 33 60 79 63 17  5 13 82 24 10 20 64
    Card 201: 71 92 68 45 33 17 99 32 96 93 | 90 82 79 26 20 85 94 61 31 84 73 30  4 87 29 28 81 27 75 39 36 58 97 98 21";
    
    let lines: Vec<&str> = input.split("\n").collect();
    let mut copies_map: HashMap<i32, i32> = HashMap::new();
    let mut total = 0;

    for i in 0..lines.len() {
        let h: Vec<&str> = lines[i].split(":").collect();
        let h2 = h[1].replace("  ", " ");
        let n2: Vec<&str> = h2.trim().split("|").collect();
        let winning_numbers_str: Vec<&str> = n2[0].trim().split(" ").collect();
        let winning_numbers: HashSet<i32> = HashSet::from_iter(winning_numbers_str.iter().map(| n | n.trim().parse::<i32>().unwrap()));
        let numbers_str: Vec<&str> = n2[1].trim().split(" ").collect();
        let numbers = numbers_str.iter().map(| n | n.trim().parse::<i32>().unwrap());

        let mut matching_numbers = 0;
        for number in numbers {
            if winning_numbers.contains(&number) {
                matching_numbers += 1;
            }
        }
        
        match copies_map.get(&(i as i32)) {
            None => {
                copies_map.insert(i as i32, 1);
            }
            Some(v) => {
                copies_map.insert(i as i32, v + 1);
            }
        }

        for j in 0..matching_numbers {
            match copies_map.get(&((i as i32) + 1 + (j as i32))) {
                None => {
                    copies_map.insert((i as i32) + 1 + (j as i32), *copies_map.get(&(i as i32)).unwrap());
                }
                Some(v) => {
                    copies_map.insert((i as i32) + 1 + (j as i32), v + *copies_map.get(&(i as i32)).unwrap());
                }
            }
        }
    }

    let all_values: Vec<&i32> =  copies_map.values().collect();

    let sum: i32 = all_values.iter().copied().sum();
    
    println!("Sum: {}", sum);
}

fn day_five_one() {
    let input = "seeds: 2276375722 160148132 3424292843 82110297 1692203766 342813967 3289792522 103516087 2590548294 590357761 1365412380 80084180 3574751516 584781136 4207087048 36194356 1515742281 174009980 6434225 291842774

    seed-to-soil map:
    4170452318 3837406401 124514978
    2212408060 1593776674 105988696
    3837406401 4016132523 278834773
    1475766470 1699765370 492158296
    3698488336 1475766470 118010204
    2318396756 2191923666 46351359
    4116241174 3961921379 54211144
    2193579298 3791037069 18828762
    2364748115 2578360543 354997036
    3085506703 3439828590 106510622
    1967924766 3546339212 219021823
    2719745151 3765361035 25676034
    2745421185 2238275025 340085518
    2186946589 3809865831 6632709
    3192017325 2933357579 506471011
    
    soil-to-fertilizer map:
    2067774073 3521970321 52706909
    3338663639 285713733 377282283
    4175452431 2125409520 119514865
    3950920796 1900877885 224531635
    285713733 3604616580 690350716
    976064449 3368036703 153933618
    2120480982 662996016 210956413
    2763248642 1355402238 545475647
    3715945922 873952429 49638562
    3765584484 3182700391 185336312
    2331437395 923590991 431811247
    1129998067 2244924385 937776006
    3308724289 3574677230 29939350
    
    fertilizer-to-water map:
    1898912715 0 159034880
    0 781591504 125461131
    4234890433 2427770485 8749678
    176481534 1845116986 384152450
    822014814 539693831 241897673
    125461131 907052635 47763268
    1476125220 244008638 19613711
    3828547378 4170474998 124492298
    2643114268 2457193301 126243103
    173224399 2229269436 3257135
    2916187764 3376015556 236473226
    764735505 186729329 57279309
    2427770485 3802085897 160735547
    2895514626 2436520163 20673138
    3152660990 2671736916 584987016
    1495738931 1131222975 403173784
    1339983969 1534396759 136141251
    2588506032 3612488782 54608236
    3737648006 2583436404 88300512
    737041056 159034880 27694449
    2057947595 1677521625 167595361
    1063912487 263622349 276071482
    3953039676 4041226796 129248202
    2225542956 1670538010 6983615
    560633984 954815903 176407072
    2847762723 3328263653 47751903
    2769357371 3962821444 78405352
    3825948518 3256723932 2598860
    4082287878 3667097018 134988879
    4243640111 3276936468 51327185
    4217276757 3259322792 17613676
    
    water-to-light map:
    527906959 2908176499 284796856
    1306013866 0 139756297
    500839409 1466481782 27067550
    1269694476 139756297 36319390
    0 778456518 2402633
    4218077327 4154765934 76889969
    812703815 4004150799 56130996
    153843304 3657154694 8975056
    2402633 905946004 132694584
    3795108796 2776082693 132093806
    3927202602 1422228955 44252827
    1445770163 1493549332 1282533361
    3794865694 780859151 243102
    2728303524 176075687 602380831
    162818360 3666129750 338021049
    3330684355 3319846298 337308396
    4154765934 4231655903 63311393
    135097217 887199917 18746087
    3667992751 3192973355 126872943
    3971455429 781102253 88826366
    1252423178 869928619 17271298
    868834811 1038640588 383588367
    
    light-to-temperature map:
    2621973104 3678827401 230150807
    1333642604 1531317439 615453278
    3364444750 2854318675 314483239
    2978187907 3908978208 107198609
    1117308885 1110453605 216333719
    1951157390 4016176817 152726483
    4168382203 2717095112 26843204
    0 312822387 5553076
    287414983 245463475 67358912
    1949095882 2597527252 2061508
    3836867339 1522015715 9301724
    648138229 2599588760 117506352
    4132690450 1486323962 35691753
    2852123911 4168903300 126063996
    2468610361 3525464658 153362743
    526108840 988424216 122029389
    5553076 0 148736111
    3265904462 1326787324 98540288
    4195225407 716774234 17303853
    181751976 318375463 105663007
    843084177 3275513023 249951635
    2214264232 734078087 254346129
    154289187 218000686 27462789
    3146382866 684048190 32726044
    765644581 2433292104 77439596
    3179108910 2510731700 86795552
    3846169063 2146770717 286521387
    2103883873 2743938316 110380359
    3085386516 1425327612 60996350
    3678927989 526108840 157939350
    4212529260 3193074987 82438036
    354773895 148736111 69264575
    1093035812 3168801914 24273073
    
    temperature-to-humidity map:
    1008510114 1939290935 27755995
    2205283444 4197517502 16218189
    1119061522 3123774174 108864966
    1566495924 221087407 33939034
    3089618547 3728555042 25452278
    2341294643 3455988869 16076350
    2286651827 3754007320 54642816
    704748216 2542375745 76754089
    445299830 3938069116 259448386
    1036266109 1300576315 82795413
    178337856 1565003866 40230920
    2122934367 1605234786 81339593
    1484902828 980285858 81593096
    2823460240 1967046930 266158307
    3827446421 1526750766 38253100
    984919715 1161567987 23590399
    218568776 1061878954 99689033
    4049237602 3232639140 223349729
    953670836 2233205237 3881060
    318257809 3472065219 89705062
    1727156113 3113814046 9960128
    3733360236 444372828 94086185
    4272587331 3688491436 22379965
    910921285 178337856 42749551
    781502305 3808650136 129418980
    957551896 2798966448 27367819
    1870217811 1686574379 252716556
    407962871 2998327877 37336959
    2508087592 2826334267 171993610
    1600434958 3561770281 126721155
    3865699521 812829188 167456670
    1737116241 1185158386 115417929
    1852534170 3710871401 17683641
    3420360273 255026441 38629788
    1227926488 2620139318 178827130
    4033156191 4250190027 16081411
    2204273960 2619129834 1009484
    2250197491 4213735691 36454336
    2680081202 1383371728 143379038
    3458990061 538459013 274370175
    3115070825 2237086297 305289448
    2357370993 293656229 150716599
    1406753618 3035664836 78149210
    2221501633 4266271438 28695858
    
    humidity-to-location map:
    2849843584 4147982382 56632112
    3849085050 3618212322 355529444
    1632881348 407047779 65646492
    3056274757 2246063521 686771203
    2729873863 4028012661 26534599
    3779070915 1543896540 70014135
    2571854216 2932834724 91402738
    2192942437 1028113266 378911779
    2960746591 932585100 95528166
    765942740 0 407047779
    2663256954 1441254676 66616909
    2756408462 4054547260 93435122
    1698527840 1407025045 34229631
    0 3024237462 156854744
    3743045960 1507871585 36024955
    156854744 3181092206 437120116
    1172990519 472694271 459890829
    2906475696 3973741766 54270895
    593974860 2074095641 171967880
    1732757471 1613910675 460184966";

    let mut seed_to_soil: Vec<(i64, i64, i64)> = vec![];
    let mut soil_to_fertilizer: Vec<(i64, i64, i64)> = vec![];
    let mut fertilizer_to_water: Vec<(i64, i64, i64)> = vec![];
    let mut water_to_light: Vec<(i64, i64, i64)> = vec![];
    let mut light_to_temperature: Vec<(i64, i64, i64)> = vec![];
    let mut temperature_to_humidity: Vec<(i64, i64, i64)> = vec![];
    let mut humidity_to_location: Vec<(i64, i64, i64)> = vec![];
    let lines: Vec<&str> = input.split("\n").collect();

    let split_seeds: Vec<&str> = lines[0].trim().split(":").collect();
    let seeds: Vec<i64> = split_seeds[1].replace("  ", " ").trim().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();

    let mut index = 1;
    let lines_length = lines.len();

    while index < lines_length {
        let line = &lines[index];

        if (*line).is_empty() {
            index += 1;
            continue;
        } else {
            if line.trim() == "seed-to-soil map:" {
                get_from_lines(&lines, &mut index, &mut seed_to_soil);
            }

            if line.trim() == "soil-to-fertilizer map:" {
                get_from_lines(&lines, &mut index, &mut soil_to_fertilizer);
            }

            if line.trim() == "fertilizer-to-water map:" {
                get_from_lines(&lines, &mut index, &mut fertilizer_to_water);
            }

            if line.trim() == "water-to-light map:" {
                get_from_lines(&lines, &mut index, &mut water_to_light);
            }

            if line.trim() == "light-to-temperature map:" {
                get_from_lines(&lines, &mut index, &mut light_to_temperature);
            }

            if line.trim() == "temperature-to-humidity map:" {
                get_from_lines(&lines, &mut index, &mut temperature_to_humidity);
            }

            if line.trim() == "humidity-to-location map:" {
                get_from_lines(&lines, &mut index, &mut humidity_to_location);
            }

            index += 1;
        }
    }

    let mut res: Vec<i64> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        for seed in seeds[i]..seeds[i] + seeds[i+1] {
            let mut final_seed = seed as i64;
            for seed_to_soil_range in &seed_to_soil {
                
                let range_down = seed_to_soil_range.1;
                let range_up = range_down + seed_to_soil_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (seed_to_soil_range.0 - seed_to_soil_range.1);
 //                   println!("seed-to-soil {:?} = range-up: {}, range-down: {}, adding: {}", seed_to_soil, range_up, range_down,(seed_to_soil_range.0 - seed_to_soil_range.1));
                    break;
                }
            }

            for soil_to_fertilizer_range in &soil_to_fertilizer {
                let range_down = soil_to_fertilizer_range.1;
                let range_up = range_down + soil_to_fertilizer_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (soil_to_fertilizer_range.0 - soil_to_fertilizer_range.1);
 //                   println!("soil-to-fertilizer {:?} = range-up: {}, range-down: {}, adding: {}", soil_to_fertilizer, range_up, range_down,(soil_to_fertilizer_range.0 - soil_to_fertilizer_range.1));
                    break;
                }
            }

            for fertilizer_to_water_range in &fertilizer_to_water {
                let range_down = fertilizer_to_water_range.1;
                let range_up = range_down + fertilizer_to_water_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (fertilizer_to_water_range.0 - fertilizer_to_water_range.1);
 //                   println!("fertilizer-to-water {:?} = range-up: {}, range-down: {}, adding: {}", fertilizer_to_water, range_up, range_down,(fertilizer_to_water_range.0 - fertilizer_to_water_range.1));
                    break;
                }
            }

            for water_to_light_range in &water_to_light {
                let range_down = water_to_light_range.1;
                let range_up = range_down + water_to_light_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (water_to_light_range.0 - water_to_light_range.1);
 //                   println!("water_to_light {:?} = range-up: {}, range-down: {}, adding: {}", water_to_light, range_up, range_down,(water_to_light_range.0 - water_to_light_range.1));
                    break;
                }
            }

            for light_to_temperature_range in &light_to_temperature {
                let range_down = light_to_temperature_range.1;
                let range_up = range_down + light_to_temperature_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (light_to_temperature_range.0 - light_to_temperature_range.1);
//                    println!("light_to_temperature {:?} = range-up: {}, range-down: {}, adding: {}", light_to_temperature, range_up, range_down,(light_to_temperature_range.0 - light_to_temperature_range.1));
                    break;
                }
            }

            for temperature_to_humidity_range in &temperature_to_humidity {
                let range_down = temperature_to_humidity_range.1;
                let range_up = range_down + temperature_to_humidity_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (temperature_to_humidity_range.0 - temperature_to_humidity_range.1);
//                    println!("temperature_to_humidity {:?} = range-up: {}, range-down: {}, adding: {}", temperature_to_humidity, range_up, range_down,(temperature_to_humidity_range.0 - temperature_to_humidity_range.1));
                    break;
                }
            }

            for humidity_to_location_range in &humidity_to_location {
                let range_down = humidity_to_location_range.1;
                let range_up = range_down + humidity_to_location_range.2;
                if final_seed >= range_down && final_seed < range_up {
                    final_seed = final_seed + (humidity_to_location_range.0 - humidity_to_location_range.1);
//                    println!("humidity_to_location {:?} = range-up: {}, range-down: {}, adding: {}", humidity_to_location, range_up, range_down,(humidity_to_location_range.0 - humidity_to_location_range.1));
                    break;
                }
            }

            res.push(final_seed);
            // println!("final seed: {}", final_seed);
        }

        i += 2;
    }

    println!("minimum: {:?}", res.iter().min());
}

fn get_from_lines(lines: &Vec<&str>, index: &mut usize, vec: &mut Vec<(i64, i64, i64)>){
    *index += 1;
    while *index < lines.len() && !lines[*index].trim().is_empty() {
        let v: Vec<i64> = lines[*index].trim().replace("  ", " ").split(" ").map(| s | s.parse::<i64>().unwrap()).collect();
        vec.push((v[0], v[1], v[2]));
        *index += 1;
    }
}

fn day_six_one() {

    let input = "Time:      7  15   30
    Distance:  9  40  200";

    let lines: Vec<&str> = input.split("\n").collect();

    // Times
    let times_line: Vec<&str> = lines[0].split(":").collect();
    let times_str_trim: String = times_line[1].trim().to_string();
    let times_str: Vec<&str> = times_str_trim.split_whitespace().collect();
    let times: Vec<i32> = times_str.iter().map(| t | t.parse::<i32>().unwrap()).collect();

    // Distance
    let distances_line: Vec<&str> = lines[1].split(":").collect();
    let distances_str_trim: String = distances_line[1].trim().to_string();
    let distances_str: Vec<&str> = distances_str_trim.split_whitespace().collect();
    let distances: Vec<i32> = distances_str.iter().map(| t | t.parse::<i32>().unwrap()).collect();

    for i in 0..times.len() {
        let mut inner_total = 0;
        for ii in 0..times[i] as usize {
            // println!("time: {} distance: {} time hold: {} difference: {} result: {}", times[i], distances[i], ii, times[i] as usize - ii, ii * (times[i] as usize - ii));
            if ii * (times[i] as usize - ii) > distances[i] as usize {
                inner_total += 1;
            }
        }
        
        println!("{}", inner_total);
    }
}


fn day_six_two() {

    let input = "Time:        49     87     78     95
    Distance:   356   1378   1502   1882";

    let lines: Vec<&str> = input.split("\n").collect();

    // Times
    let times_line: Vec<&str> = lines[0].split(":").collect();
    let times_str_trim: String = times_line[1].trim().to_string().replace(" ", "");
    let time: i64 = (&times_str_trim[..]).trim().parse::<i64>().unwrap();

    // Distance
    let distances_line: Vec<&str> = lines[1].split(":").collect();
    let distances_str_trim: String = distances_line[1].trim().to_string().replace(" ", "");
    let distance: i64 = (&distances_str_trim[..].trim()).parse::<i64>().unwrap();

    let mut total = 0;

    let mut flag = false;
    for i in 0..time as usize {
        if i * (time as usize - i) >= distance as usize {
            total += 1;
            if !flag {
                flag = true;
                println!("time hold {} time diff {} time {} distance {}", i, i * (time as usize - i), time, distance);
            }
        }
    }
    
    println!("{}", total);
}

fn to_tuple(s: &str) -> (&str, i32) {
    let split: Vec<&str> = s.trim().split(" ").collect();
    return (&split[0].trim(), split[1].trim().parse::<i32>().unwrap());
}

fn get_hand_rank_vec(hand: Vec<&i32>, j: i32) -> i32 {


    let mut sorted_hand = hand.clone();
    sorted_hand.sort_by(|x, y | if &&y < &&x { Ordering::Less } else { Ordering::Greater });

//    println!("sorted hand {:?}", sorted_hand);

    if (*sorted_hand).is_empty() {
        return 7;
    }
    
    if *sorted_hand[0] + j == 5 {
        return 7;    
    }

    if *sorted_hand[0] + j == 4 {
        return 6;
    }

    if *sorted_hand[0] + j == 3 {

        if (*sorted_hand).len() > 1 && *sorted_hand[1] == 2 {
            return 5;
        }

        if (*sorted_hand).len() > 1 && *sorted_hand[1] == 1 {
            return 4;
        }
    }

    if *sorted_hand[0] + j == 2 {
        if (*sorted_hand).len() > 1 && *sorted_hand[1] == 2 {
            return 3;
        }

        if (*sorted_hand).len() > 1 && *sorted_hand[1] == 1 {
            return 2;
        }
    }

    return 1;
}

fn get_hand_rank(hand: &str) -> i32 {
    let mut m: HashMap<char, i32> = HashMap::new();

    let mut j = 0;
    for c in hand.chars() {
        match m.get(&c) {
            None => {
                if c == 'J' {
                    j += 1;
                } else {
                    m.insert(c, 1);
                }
            }
            Some(v) => {
                m.insert(c, v+1);
            }
        }
    }

    let values: Vec<&i32> = m.values().collect();
    let v = values.clone();
//  println!("hand: {} map: {:?} rank: {}", hand, m, get_hand_rank_vec(v));
    return get_hand_rank_vec(values, j);
}

fn compare_to_sort(hand1: &(&str, i32), hand2: &(&str, i32)) -> Ordering {

    if get_hand_rank(hand1.0) > get_hand_rank(hand2.0) {
        return Ordering::Greater;
    } 

    if get_hand_rank(hand1.0) < get_hand_rank(hand2.0) {
        return Ordering::Less;
    } 

    if get_hand_rank(hand1.0) == get_hand_rank(hand2.0) {
        for i in 0..5 {
            let card1 = hand1.0.chars().collect::<Vec<char>>()[i];
            let card2 = hand2.0.chars().collect::<Vec<char>>()[i];

            let card1_val: i32 = if card1 == 'A' {
                14
            } else if card1 == 'K' {
                13
            } else if card1 == 'Q' {
                12
            } else if card1 == 'J' {
                0
            } else if card1 == 'T' {
                10
            }else {
                //println!("{}", card1);
                card1.to_digit(10).unwrap() as i32
            };

            let card2_val: i32 = if card2 == 'A' {
                14
            } else if card2 == 'K' {
                13
            } else if card2 == 'Q' {
                12
            } else if card2 == 'J' {
                0
            } else if card2 == 'T' {
                10
            } else {
                card2.to_digit(10).unwrap() as i32
            };

            if card1_val > card2_val {
                return Ordering::Greater;
            }

            if card1_val < card2_val {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }

    return Ordering::Equal;
}
fn day_seven_one() {

    let input = "K99QT 53
    TKQ7T 320
    22A7J 490
    267J9 69
    665JJ 431
    K856J 605
    977K9 552
    KKK8K 285
    53697 370
    3J337 879
    84A9K 901
    K4289 211
    4JA46 100
    K452K 315
    3K48Q 239
    99TT9 985
    A7JA9 582
    836AT 770
    J2KKK 61
    KK7KK 37
    QTAJK 351
    T4578 198
    TK55T 513
    44K9J 853
    KQT58 203
    66A6J 881
    K77KK 31
    6666Q 410
    8J8KJ 522
    69KQ7 594
    T27J3 110
    44999 461
    98885 231
    55655 372
    43343 958
    82247 300
    33336 957
    59JT4 303
    AQAJA 691
    J3626 696
    9379A 788
    JTTTA 795
    442A6 989
    73933 857
    979QQ 817
    65566 150
    6T565 182
    4T5J5 9
    TT3TT 147
    J8338 229
    6AQ8Q 636
    79KK7 933
    8T6TT 145
    TTT4T 177
    KAK52 952
    9K83J 469
    55855 718
    TTT5T 209
    9555Q 309
    AA3A5 327
    647T3 310
    22234 717
    5TT9J 5
    T5A86 831
    3J333 544
    66767 571
    9A777 269
    43QTT 822
    888J5 422
    T4435 259
    75394 17
    KJA2A 95
    A2KK4 762
    4T73T 595
    7Q97Q 3
    9J998 964
    KTK2K 267
    87Q87 375
    4744T 909
    3TTA7 637
    6J767 409
    3KT7K 588
    7T6T6 302
    39959 887
    TKTKT 144
    K99K2 777
    48237 403
    K93A3 886
    369AA 350
    43QK6 193
    657KA 199
    7398Q 314
    77778 38
    ATTAA 903
    A9A93 112
    T8TT4 128
    776K5 509
    868T9 641
    6T5J5 910
    TQ722 976
    9A9QQ 224
    J93J5 948
    JTTT7 576
    JKAQK 184
    AAAA5 764
    J3939 262
    7K7K8 204
    88928 663
    T8T8T 352
    5KJQQ 477
    22TTT 833
    299Q9 437
    67J42 393
    3A73A 961
    982K7 614
    J55Q5 549
    99699 741
    4T4T6 366
    55522 256
    39898 301
    9AA49 137
    9A6AA 706
    Q6668 740
    322J2 716
    K5454 439
    Q3Q77 954
    78J49 946
    66828 242
    652A9 340
    QQJQQ 342
    2Q2Q2 92
    2K352 871
    7Q5A9 361
    QAQQQ 925
    QQA26 625
    97J3Q 932
    KQ965 675
    8J88T 738
    JK84K 19
    AA333 130
    TTTKT 536
    7K8Q3 902
    A4A5A 529
    393K9 192
    85558 216
    TQQ77 4
    A5AQ9 276
    62672 90
    246T8 674
    Q424T 436
    474K4 970
    733J7 156
    73T23 713
    K864J 676
    K6K6K 75
    5524A 270
    AA7AA 89
    A6764 105
    JK555 103
    3Q322 124
    89K36 66
    7J724 686
    KJKJK 939
    Q7793 219
    J5A5J 296
    7AATK 666
    4442K 429
    4J734 504
    28TQ3 668
    9T623 862
    7Q7A5 810
    468A7 517
    44Q8Q 569
    6TT6T 36
    69965 824
    2QT39 864
    585T5 744
    84888 865
    QQJ6A 702
    66656 365
    K9T38 377
    56576 604
    95665 516
    96496 657
    5KK6K 274
    23857 982
    855J9 13
    T25QQ 264
    Q9K2K 354
    37K73 631
    88A57 487
    689KT 971
    898QQ 701
    2QA2J 482
    99J97 427
    77A76 167
    7TK93 596
    34647 579
    46J88 460
    QT4AT 380
    8T488 553
    32A44 726
    J3332 635
    K2J42 897
    93339 55
    6J7Q6 311
    Q9QQJ 7
    47444 539
    93399 567
    ATJAT 77
    TTQQQ 859
    9Q999 281
    K752A 412
    87977 205
    92334 995
    79QQQ 26
    JJTA8 581
    A4J24 444
    4J444 921
    Q9KK9 255
    45K2A 728
    KA5J6 621
    7J277 850
    56884 67
    44JQQ 240
    67Q7Q 426
    QT6T5 265
    AA9A2 252
    A9KA4 649
    TTQTT 554
    4A444 698
    95586 70
    7J45Q 996
    48848 500
    Q3AAA 474
    K3A7T 980
    5655J 977
    75J92 577
    TA94A 951
    336T3 400
    442QA 761
    K2J53 537
    JQQ22 470
    AJA53 88
    73797 672
    657TT 869
    2TTTT 142
    JT49K 227
    A7A77 338
    5J2Q8 221
    92269 664
    7A965 307
    3355J 308
    J9T98 518
    69TQ5 899
    57455 355
    AKA5A 489
    79K77 39
    72767 298
    AA858 695
    K3JTJ 283
    4KKJ4 258
    7K248 704
    Q9J47 617
    3QQT8 820
    87J3Q 540
    9A6A9 438
    3AT7A 344
    5656K 478
    66663 385
    K2KQ2 572
    37747 612
    58T72 690
    Q47A9 368
    8K53Q 687
    99799 856
    TJAQ2 476
    KA4A4 1000
    Q634A 408
    TKK88 785
    829JK 633
    26446 132
    JJJJJ 512
    T666T 51
    QQQQ3 751
    5A35A 534
    2222K 432
    5K72J 378
    866JK 566
    5T2AJ 11
    KK5JK 898
    97T2J 163
    7757J 247
    33237 640
    T554K 975
    K666K 940
    2T272 91
    7A988 960
    T9736 671
    999K9 68
    95595 802
    A5JJ3 652
    78829 867
    4JJJ4 148
    AA84J 541
    4T4T4 563
    A334J 931
    9T29T 416
    96845 992
    A22AK 555
    224KK 584
    84484 271
    22A88 986
    37AKJ 49
    JQ6QQ 284
    33853 280
    TTJ77 935
    66J66 720
    468A3 185
    QA958 642
    888J8 580
    JQQJQ 560
    J29J9 57
    6888J 14
    QJ8TK 440
    AQQAA 136
    AA9AA 823
    6ATTT 175
    7T42A 402
    A7J9K 907
    53A87 805
    26262 930
    22262 507
    27JK4 854
    QAQTT 731
    8TKTK 418
    TTT44 244
    J45TJ 98
    86898 681
    444AK 929
    KK88K 521
    8356T 644
    TKT6T 401
    Q99QQ 415
    96299 659
    8T888 598
    5T955 888
    85572 658
    T59K7 471
    7Q77Q 927
    AAA7K 949
    TA55A 278
    KJJK8 692
    AA5J4 638
    AKK4A 861
    77767 774
    K4KKK 143
    TKKQQ 10
    Q598T 794
    4A44J 201
    288QQ 602
    3TT2T 80
    Q5855 955
    AKKKA 43
    66QQQ 942
    97977 29
    J444J 157
    3AAAA 273
    3J988 420
    JAAAA 715
    4KQJ3 250
    KJ4K3 480
    QQ3Q3 113
    3883Q 421
    6T4TT 44
    77373 472
    4K74K 76
    AA226 943
    4Q4Q4 305
    7J729 763
    Q4649 456
    TKTJA 966
    6JJ24 131
    5K855 101
    AAJ3A 743
    82Q5A 558
    854Q8 186
    TTTT9 74
    AKAAA 323
    KK5K5 556
    3A69Q 164
    2AAAA 404
    Q4QQ4 162
    7T577 313
    5QQQQ 618
    87535 266
    4A4A4 356
    875J5 33
    9K225 479
    JQK95 941
    KQT28 826
    53356 760
    QQ5Q5 16
    696J9 498
    TJTT6 836
    7A7TA 979
    999AA 170
    55K5K 523
    733T3 746
    39929 503
    686AJ 885
    QQKQQ 667
    A8KK8 96
    Q555A 446
    Q2QQJ 797
    QQJQ8 445
    6T6J6 583
    65565 451
    535A6 765
    45QKT 453
    A26K6 816
    J4JJ8 60
    35535 510
    T85KT 613
    9Q3Q9 358
    53425 680
    4JA45 878
    76328 481
    QJ2AA 813
    QK4KQ 324
    K9KJJ 919
    22822 734
    JJJQ9 441
    33393 928
    29299 495
    69666 825
    T5Q82 423
    43687 834
    TTQ6A 682
    7KQJ6 736
    TK2JA 65
    665Q3 232
    A7297 593
    Q6Q33 837
    522J5 272
    44424 891
    57T63 41
    222J9 722
    99797 228
    33A33 711
    5TT7K 458
    KTTT4 22
    4774J 430
    9J889 257
    88483 225
    KKKKT 678
    7QQQQ 322
    37337 607
    QAQJA 997
    46696 316
    4A4AA 924
    QQ3J3 835
    72Q72 236
    4AAKA 254
    T989T 557
    K7Q76 115
    77JT6 591
    683A7 390
    A97Q7 295
    AA8AA 895
    4KK5K 660
    J5665 155
    JAJA5 920
    3T333 806
    5TJ48 287
    J4434 332
    98TQT 492
    J539K 781
    9QQ94 452
    TA593 745
    A4JA4 515
    2T332 609
    2KKKK 494
    4KKKA 30
    Q254J 590
    4T28A 608
    T9Q7K 275
    48843 279
    9629K 526
    778J7 329
    J67K5 697
    26A22 220
    67J97 849
    84Q88 863
    73777 248
    34KA2 286
    A9A9A 126
    Q2457 984
    55672 106
    77T89 165
    TQQ44 413
    KKJKK 497
    Q9482 417
    88999 550
    TTAA5 911
    TK2T2 790
    44445 578
    J55T5 169
    T738A 847
    55554 789
    AJAKQ 829
    4Q492 394
    QT5Q6 433
    Q5Q55 917
    KQAA5 801
    QQ242 870
    92A76 334
    7TT96 610
    J334K 520
    T333J 527
    J464K 149
    QQJ8T 349
    2KAKK 561
    K3JK9 40
    TTJJ8 213
    57557 151
    9ATAA 434
    JTTT3 990
    TTTT7 998
    6T3K3 384
    45738 293
    999TK 288
    999JJ 312
    333TT 15
    8A7TK 650
    35JT4 784
    29889 796
    TT669 114
    8585J 502
    75555 772
    5JT59 173
    AJAJA 99
    5842T 326
    8TA26 214
    99T29 776
    8TK44 896
    74J48 730
    78888 123
    3T763 139
    TT8T9 548
    8Q737 405
    2AK5J 684
    8885Q 804
    33Q36 183
    2K935 987
    3Q55K 346
    36933 543
    K575K 435
    94847 947
    7K666 107
    9TTTJ 367
    KK636 799
    J7TTA 766
    74K74 484
    Q5445 945
    2A662 48
    886AK 525
    A2555 395
    26J66 188
    22333 771
    J2KJ2 533
    32329 330
    4T4TQ 455
    836KJ 318
    29999 830
    KKAJT 905
    QQA9A 673
    33QTJ 179
    AQ748 719
    4Q3T8 46
    AJ79J 978
    44Q48 524
    87277 347
    2JQ4T 611
    Q6868 773
    A7355 950
    89Q68 263
    4947J 1
    QQA7Q 134
    TK32T 138
    45544 838
    J5QQQ 936
    7T656 488
    KQQ8Q 111
    A6T95 122
    24T4T 291
    64323 646
    4JQ4T 268
    Q9J82 851
    9JQJQ 710
    22T2T 606
    9J969 564
    36633 735
    2222J 972
    2K77J 292
    2T836 406
    9J379 721
    2T3T3 253
    55Q55 875
    KKT3T 197
    754JJ 643
    25577 261
    96776 81
    33Q33 756
    4455J 723
    274TJ 506
    43432 118
    J6JA9 634
    68Q2T 923
    JTTJT 47
    44777 215
    K5K5J 937
    52252 391
    T6365 883
    JT4JA 808
    25K9A 336
    6TTTT 876
    67767 703
    88485 160
    AAJ8A 814
    Q4K66 798
    853K6 873
    K943A 135
    74T8A 732
    J8T77 780
    Q32A6 246
    733AT 965
    68536 108
    K8K88 815
    75A42 234
    9Q24T 967
    J4AAK 514
    77KK7 397
    89889 93
    KJAKK 129
    8KA88 973
    488A4 62
    555J5 226
    3AA6J 153
    35239 679
    22694 574
    J3344 450
    9J999 545
    TKAT6 768
    57JQ9 969
    22882 189
    777A7 24
    33323 858
    J85A9 260
    TT2AT 993
    KK29T 200
    55595 821
    A5555 615
    82243 125
    82338 586
    TK385 457
    QQ89Q 243
    7T98T 202
    34K37 597
    8889Q 508
    KKTKQ 289
    TTTTJ 944
    QA8A9 622
    33KK3 277
    2J233 630
    K5666 819
    4J824 812
    7888K 45
    K42K7 846
    23A49 485
    98888 337
    2JJ22 468
    966A3 662
    J4JT3 86
    8KJ33 369
    KQ3A8 237
    AAQA6 904
    JKAKA 538
    K9T43 968
    44745 34
    8A4J8 78
    K523K 465
    JKAK6 294
    37J77 207
    33TTT 988
    A8TA8 463
    Q56Q8 194
    QQ443 319
    382Q9 532
    J66J6 568
    A92T4 388
    K5J49 172
    84288 428
    2QK99 752
    5QT27 767
    37Q54 592
    TAAAA 475
    AA9Q9 868
    97T3Q 58
    TAAJJ 120
    7TQ85 360
    45585 119
    JQ252 793
    39229 655
    66JAA 133
    67276 233
    66646 493
    A8A89 725
    K5K4J 620
    79KK9 818
    AA4AT 218
    86888 685
    3A252 353
    T4TAT 589
    TJ269 190
    3366T 449
    J87Q7 800
    77K77 656
    Q5856 628
    9TT33 874
    8J8TA 158
    33335 35
    4K5J5 587
    87877 104
    9A66A 168
    77277 843
    52545 570
    2QA3K 841
    72K75 727
    A8AA3 535
    25J5Q 729
    T3939 363
    4K66J 832
    99989 845
    36Q53 916
    TTKJK 462
    764AT 963
    88328 934
    9229K 708
    2J886 328
    97A38 448
    799J7 755
    5AQAT 749
    7KK76 146
    KAA2T 109
    2222T 882
    32257 117
    3KQ3K 6
    AQAAA 54
    599QQ 442
    44423 809
    TT99T 84
    523Q2 501
    33433 884
    2J894 918
    44484 411
    5J9QT 926
    J7872 63
    K2KK2 981
    655AA 217
    9949Q 42
    4ATJQ 991
    2294J 811
    K32JA 79
    5T84Q 839
    A8K24 530
    57T67 333
    JQ4TK 791
    Q58Q5 235
    74756 627
    62TT6 742
    34443 208
    7Q7QJ 102
    9Q6TT 792
    TA797 72
    67K56 645
    5K5QQ 629
    62666 669
    6T5KT 348
    66K32 531
    J2AAA 331
    4KJ7T 223
    T88K8 191
    55244 714
    A4J57 983
    Q777K 251
    QQKKQ 71
    A8858 196
    4J8JK 387
    88636 386
    32J56 230
    6Q833 23
    TT8T2 877
    A8JK4 483
    22A24 600
    35555 166
    4AKQJ 748
    94448 52
    92229 842
    292J3 912
    6J4AA 783
    A85JJ 890
    64556 210
    QTT9T 304
    7KJKK 180
    7777T 915
    59TA7 238
    TT8TT 206
    44434 953
    33373 906
    2J57A 828
    6QJJ7 889
    9KA9K 27
    T22KK 651
    KKJQK 852
    Q7373 775
    9298J 994
    TT739 654
    88787 491
    J68QJ 661
    K8QK4 893
    25T25 381
    2K895 152
    J777A 626
    62562 364
    Q64JQ 32
    98337 28
    4A7T4 299
    36J33 245
    83883 639
    59495 962
    66T6K 127
    JTJ2T 779
    KJ4Q6 424
    J822J 306
    3KQ45 362
    25QQ5 807
    Q8TJ7 389
    QJQ37 94
    T2QKK 747
    25555 632
    Q6Q69 64
    J2767 737
    333JJ 195
    9A9A5 689
    823QQ 707
    8T5T8 443
    23AK6 59
    86446 892
    93964 8
    QKKQ8 97
    59T86 866
    Q3QQ9 677
    55344 414
    57KJ5 860
    337A7 575
    9JJ9K 699
    3AA38 343
    37273 85
    22AAA 154
    A9896 601
    45J97 18
    T7689 599
    55537 573
    JTQT6 473
    262Q2 616
    8J5QQ 486
    25444 705
    2K2K2 922
    93QK2 20
    JT9AA 396
    83JTK 757
    J9AA4 827
    J3Q44 159
    922TT 505
    Q22A9 803
    3753K 700
    T5545 222
    777QA 840
    2T987 908
    JJJ34 547
    J9Q85 187
    34994 880
    K7AKJ 670
    9QQ4Q 647
    522JT 855
    K8K58 750
    55JJ5 653
    J3977 249
    KJKJT 419
    8866J 383
    56AQ4 87
    5444A 392
    J77J7 345
    495K9 848
    4J449 844
    96T66 999
    66628 758
    66684 171
    47767 454
    TJ54A 21
    58888 778
    J8J88 467
    96JKK 585
    8A5J5 624
    2A222 603
    44494 759
    J3A9K 956
    8854J 141
    5JK22 341
    T5Q25 359
    K3533 693
    44294 914
    JJJ7A 382
    9K654 241
    5A44A 900
    777JQ 12
    A6AA6 176
    T888T 399
    AJ77A 739
    J636J 499
    55KQK 551
    79JK9 447
    A888A 528
    45T49 466
    683T4 459
    KT7KJ 140
    88855 282
    85T7T 753
    542A8 496
    96554 769
    45A46 56
    642K5 511
    87AAJ 297
    3T28J 754
    A266J 709
    9QQK2 83
    T32Q2 425
    777J7 398
    9A5AJ 373
    3958K 683
    9AAJA 733
    92Q9Q 665
    777T3 786
    529J8 290
    592TA 371
    9Q99Q 787
    8444A 913
    AJKAT 872
    95K99 174
    KK4K3 339
    34A73 694
    25222 374
    Q7Q7Q 325
    32T54 321
    53K9Q 542
    A8TAJ 73
    4242J 648
    27772 161
    2K242 335
    3J6AQ 464
    3999K 407
    52JTT 619
    688J5 959
    2TT6Q 376
    K83AJ 379
    372KQ 357
    A6T96 25
    65AT2 565
    AAA6A 181
    J8JJJ 623
    3Q3Q3 559
    K38KK 50
    72227 317
    537QQ 546
    99KK9 212
    5J7J5 121
    3QT42 782
    9QQ32 116
    JTKK5 724
    J99TT 938
    AQQQA 688
    7729T 519
    8TKQ3 712
    53447 562
    63679 974
    TTTA8 2
    9K642 82
    44T44 894
    T9TQ4 178";

    let lines_str: Vec<&str> = input.split("\n").collect();
    let mut lines: Vec<(&str, i32)> = lines_str.iter().map(|s| to_tuple(&s)).collect();

    lines.sort_by(compare_to_sort);

    let mut total = 0;

//  println!("{:?}", lines);
    for i in 0..lines.len() {
        total += lines[i].1 * (i as i32 + 1);
//        println!("rank: {}, bid: {}, bid * rank: {}, total: {}", i + 1, lines[i].1, lines[i].1 * (i as i32 + 1), total);
    }

    println!("{}", total);
}