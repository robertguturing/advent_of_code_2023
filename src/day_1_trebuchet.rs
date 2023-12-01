fn combined_number(first_number : char, last_number : char) -> String {
    if first_number == 'a' && last_number == 'a' {
        "0".to_string()
    } else if last_number == 'a' {
        first_number.to_string() + &first_number.to_string()
    } else {
        first_number.to_string() + &last_number.to_string()
    }
}

fn coordinates(unsanitized_text: &str) -> i32 {
    let mut total_number : i32 = 0;
    let mut first_number : char = 'a';
    let mut last_number : char = 'a';
    let length = unsanitized_text.len();

    for (i, c) in unsanitized_text.chars().enumerate() {

        match c.to_string().parse::<i32>() {
            Ok(n) =>  if first_number == 'a' {
                first_number = c;
            } else {
                last_number = c;
            },

            Err(e) => {},
        }

        if c == 0xA as char || i + 1 == length {
           total_number += combined_number(first_number, last_number).parse::<i32>().unwrap();
           first_number = 'a';
           last_number = 'a';
        }
    }
    total_number
}


fn main() {
    println!("{}", coordinates(TEXT));
}


static TEXT: &str = "9dlvndqbddghpxc
rtkrbtthree8sixfoureight6
fdxrqmfxdkstpmcj7lmphgsmqqnmjrtwo3tcbc
onetjcsmgk57nvmkvcvkdtqtsksgpchsfsjzkkmb
six8threepvlxttc85two
8five9ttqst2one2vz
hbrmhsnjeight64dgdnvdbspk7ninetzbvjczqrj
fourtwofivesix5
3gksfourqf48
7one1tnqxfvhmjvjzfive
sevenmcjs3lmlmxmcgptwobjggfive6four
seven8five3
5sfknxsn5sevenfour446
bxc5two67seven2
jcsfivefive89seven85
nine296
seven5twoeight
1eighttwo8jfnhmfivefivezdsxqxqsjkone
foureight48sbkkvc17zbksgvcbb
lnzgspccsn4cxqqdbkj
qlxrxkpeight48xbgqnlkpkoneseven
z7onetwonec
7cns
pnpfninefive79twoone7
2hrqpjjjbn
4gmlttgdzrhxbxnnine
4sixfiveone76jctmjsxdh5jrkv
3kvjmhpmglrdgmdnine
four63sevensevenone
jmz1eight4threej1
four21zxksf9jxdvjmtn337
msnronenine43three1threefrv
rjfhd6eight4
78blgveightfiveone7bnsfnrmxsmtwonemrb
sixseven6four6
mdjphcm9
xsjmgdgqtwolg1nine45eight
five2six85npdqxgrshdjs4
jbbnine2ttrktc2hxpxfdxgf
fngvqsgmjfmfslrmone2vtpsstpkhr2jmmxk
f683glvfsdvnsghvrzcdmxnx81
lqblzgj322kqfsjrbxgcgsct
threeldfnrbstbxqdpxpkbztbp84eight
sevensevenmthprqg9six
qldknljthdjthreeklttd6six
7eightcdqxcftbgbfbnvqfive
gxjzhvkbcjhscdxhjdqxnhsevenxrdrjbcl5fvlvlxjjvb
9sixqnine9jk9six
zjtdbzr6njdgflrmpshxn
rktpknvmjknb7threefourhdxhcdtgtkvone
276lzxhone3two2
82fivelppqzjq
lchslxtwohslsztgps5pdssctclhdkqtwo
2five8three9dnine8
68jpnqldjgfnpcmvbxnszhz2252
q32
2sixtwo87
hlmdvlrqlrjdshone3five
fivetwo7
xsdcktrone29
eightfour9eighttwoxvhdth9lndg4
two8jgddjhcj67eight
8nrkrcrqhr21stqtvqn
tvbonepzrrklninexmpxrlkcpgg9qgrkcjt
6two5qjmoneclfhzhkxbntmvmdrc
prhmt4xvlg
3mghfgrhzkj5
lq2lnrcj1pnlh
sixqhfqrmf8fivenkkcqpgf85lone
tlrlcbhdvd5
three8seven
thfns325threefpvlntfvrf
twoblkldr2mmrsxpqxcms39seven6
2mpcvttntg31mkznplfkbcgccsix
2sbs
828jnvjnbgrs1
fhcglnxzss23bxfnpczvthree2
7fhghprqvrbx1nxml2one32
5fivethree7qrsixmcdveight6
five4seven
1cfcfdzfjphsevenmdvnzh
qhmnleightbhbcntwojjfxpvlxt2spponenine
fivelskzvzsix5xtqkfl1
2jfgpmdncvpjmqcgvnzpqlstzgfdvfxrlscjkzczsf
rzzplmzsfivetxbhcqnzdq4
mvrvfour9eightseventhree
7smrdqkrh8qlzc
mpf2fivefivefivepgm
1sixpkrdjlszgdnccnllfsevenksdkvqbxbpbblthqpzqf
six91dxxdhrxcbmqpqm
sevenfour8nine7
one2hnfvh
55jmqkqgvsgqcrzvmzqhone8twopsrtgmqrj
xnfjxqlrsjmgk68kvpptczxhkxcvrpgctddjklrmhzjgtcjh
seveng6two
1threexrzqcrknhc3
58qtpqqz58888cmhs
pxvbhmczrvpnjnsrcdrnrjvzzpjnbgbxdseven6
34two565
four4six
sqtxsjsix54
four3xrxmrkn4nrcsmljqrninethreeone
3six4bqddfivejnfrhnqqsgqhj
gtbtrtzp8seveneight3seven4xzdnfrvzgn
vggfdfvlrgvqqvjhkmxfsfbdpqfivexs16
onegkvdhrfninerndk46nine
5fiveeighthnrlzln
threeqctjkpxjx39six89two2
99ninecbzck
nxqlhpgnine6pvrrpfjpssix6seven
1dbrzjkckthm5sixsix
fivefmfqcsj19nhnzg
five91eightninecn
7ljnl71eight7mzhzfht
84sevenzrqvkxszdhg66
jskktpm5mphd1
boneightfournq6ndnqpdbm97five
5twonine
scqpkxrjtwo8foureight11
3eight7brgqfivezpgclhfj
zkfrsdgbmczlrzchvfql78ftsdqk8vmqccbn
oneseven3kgdkjzkmq94
sgtwo59kdmhbndzd
threeone37
mqxvrhmrpqnltvt9lrpplmttkhdvtln9
onecxqvr48six2
pjnx2eight7five
two189one6gbqvllzb35
psvxjhscstjfkbpxhbbb4zvdjbcdxqrqzqlzp
n18hmhzrqjrpcxztwo
6two46zblgrbmjcqbnxqcnbf
7kxsjdqcmxrvmdtscvxgrnhrmrfour
onedzeight8qfive2
seven9bkjone2sixqnztq21
fiveninesixfkzlsn8fivembfjnx4threetwonexb
5zmxtcmzqqdthreetczccnxhkxrbntmfoursixjhhrsdxthree
1four2xpkfgcn
88trnvjtqsmseight8
pbbpbfgsrst5five
jvgvdseven2two
962sixoneonectfgpknl8nine
rkbnzz1l42eightflb4
twoxfll2fourbjfjgxfbtk
35fourtwo
lq56ninefour1three
77ztlmqxcxrj745
nnvqrthreedt2eight6hvrlnpbts
6five6225sixfnzzbh
nine1three
8xgdsdfgcfourlhn
2three3ninelckpddbmdrfournine
89zmvxnlrj7658kjdjchq
pfhbgpb66twogpn7twobpvrbmmrvp
8hgqmztnmhkcr6xrxrbhj
6fivemnfcvvx8
3mxpvgzq9ninebmlktwo
thzhbsl49seventhree1bdxcrgjq
rlcfour3ffkxxrhb
rkj2eightfive
xqncfnhkcqxqjgbsjhnrgm6
ninefour52ninefourtworhsix
lpzcmxt97mlkjhlcone
two4qkmqgrpltkrdsctpnsqmbtptklprx6ncfpseven
61shxgxdqqqzngnnzeightkhmgrxprb2sixjx
9eightfour3one6seven
eight4sixqjxdjnzmkfflpfmkf
3bfc71
14sevenfivezzmt8cpptl
1sgrzdqdndsevenninembzvfive
lxftrbfcqfoneeightrsdxlnp1x2jsgn2
vlfcjfourmtxbcngpjvkcctvbgkgpvvqpsg1
116five132seven
eightpvfsfbfzjcdcvh8kbrcz357
99two
four6six
fzzklmnxvfrhd62xmftbrhgsslrlqv
5six55zbdlgc
62nineone8qcglr1
sddddseven55fourlpqzbgzfive5
eighteight17
nine98
three9qqxgfpjfkeight81nine2
twotwosevenvkzzhrpgninecqvf9
lkhf5onexsrtwo
8hfzvnzrd
9threeninethreeseven
onedpgjzsixxs4cg5jlvzcsbd
fgmfive97
4fivefourckthree
nine1fivefour
7qlchtvd
tmtmtnxdpsvfour4621four6
5qfvzh7seven
fourfivebqnpzbg8three
34kd9four5seven8three
kcqcxzmnkdt5twojdggp
4lvbfdpzjsdthreeldvkcbqrspktwo
2nine93foursixnine5
8tbpjgmxltwojlpbnsvqhsjfcjcfvcrjqppdb5seven
8twodcpglrxcq23sixfour
3sixvczzqsfive
1three45jh54tbdvkj
pdrzqxdtcnbrnine8zvffmrtwo11
3eight54sksqfxhzcdfour
7hsllsjtxtwo9jhjlhthreekdfssninetcjjrm
9sgmnine1kjmhjthree
nineseven9five
ggdcphlstwoonethreeffgmrseven2xsbsf2
75ssbccflrrf4lpmptcqjcmfbpklqc
5six2threedvvlxdxsixvxxghpddn8pbnr
tlbjnlxfxvnine4grhj71hnf6
964eighteight4eight6t
5qbmnsone1fourseventwo
oneseven1five9two
5fdpl37vklxpth672
4czvrnxtlfiveseven74
qdgqbsd5fourfive
ddrqjnkbq87six2fourmtghdtvtl7
zdldjnfxms692rbdfgvtsevenxzfjpnsf
nsqtkbbfhn8threesix3fivefoursix6
sfxjzhpqqslfourmpph44threetwo
onefive3seven21
9eight1sevenfive7nine
7twonrthgr73vzb
18onefourhxxm
3xvboneightlb
3ftqct9
8five55
4eightmzrlfjqqddffgmfl
74mhzvktwolgpvrvnphc
sixnine694ninetwo
bzpnktvcqleight2
5oneeight1five2dszthreedncqzmhc
eightfrbmcblrghgmpkrdnh7fournzbvtrzvhchsix
threethreeoneone6kbghfst
4fiveonexfchmclqhqfive
1t3nine
57jclthreefourbtdgsggzh
qpc9fhddthreefive
9eightnine7cgpbbnine8
88twoxbzjp6fmqlznzgpgdqmmnpmkvctm
threelb9four168qpchgnfn
4onekrlmslkd7five
4jfxtsseven
zsthvnxpf7
656three
453seven
zxmr55hgnvtjbbqhfsxssjxzshcbkvsbzg1
pbkffqzrbvxvqjfgkpmp5
ninesix677lpqpgmc
sixgmb67fzdhnnfk7
jlgjbltbtwotwoeight9sixninevlngvvj
xdxqdhps9sevenzmzdlnvsix3oneightj
ninetwokpzvvqlgtcfx2
dqmjchlbj7sevenfive
61eight9kjmxclvdrdmvnthree
8nine6three
3ndpknone
seven4one
8dxljzpk8twoxfmhbqnmqeightdhxvpgxcnine5
6five3seven
rzgfivenxhpr6gxqflrskhjhvoneonegzbclk
jrvjhkfv7sqnsz86five5skvvmspvjd3
sevensix3c6llkmsmmssf3
sevendf73eight74
7bqbbvmq2krkbhlsh
3dqq55eightcptwo
2bjgfrgrtgnfour
44xznjjvlhzclsix
seven6rtzponeseven
6ghvvkhrzvjzpxlbkonejlmrhq
zgsxthree6
3crzdvbnhtsgldjvbpsixbjmlxsv3vrzjtv
xcpnbsevengpffknsv84sixsixseven
four898xkqnhspbqmtcs
5twonine
six2ltwosix
cnine22sixeightwost
6lx63two
fbqoneight3five
rbtwone2eight8
9fivezrsdfkdlqone1nthjvdlmxhqvj
pmmfp8seven
1qmttlsjpxjbsrzmbtlxqzfzvrs968
fgdonexgvvnine5jsm
rbgmbtwo9fsntcthreesix76
fbrsxxshseven3gtv
mkxfour66one9n
eightnine1fivecsixlqds2
lhxnxhfnmlmhxkcxndmnine1kfpb5eight
6eight13rseven6
hxcfone64ninesevenbgsnrqppqmnnineeightwof
threebjpbtpzgx5mnthreensixoneightz
bpdvfqlzthree2vvcmvzllqfbgjgp
two926sixeightdsvdmnxthreetnqvktdbn
bhntwonefourninejjcmlfphzpseven95
prqoneightseven1nbcxnqjfourfivecmj2
stwone1oneninevcrfzpfourfivetwo
ztkknhjr9six8
twomlvrzm934seveneight
ntnn1bqgbxf13rqdqtcrbpeighttwo
seven58ninemdqonecrkbdblone
foursixthree1oneeightone62
8385dhgcqjtrsixtwo8
blpzxstwosevenqbdhkb8sfggkbdhx5rzftqsf
mqbeightwoninefourlgcj9
njvpkpvjpmvmbsrdgdvx7frrgvnfrmmrfrhqdtwothree8
rtwonesxxone8tnine
2xqgrqmone73five
5ggzdxvptwonineeightdj
svqhzhzbsmhljxeight7hljd
one81kmnmpmfkseven
9ninefourbvbpljb3nineqxnkgzgj
sixfour7nvdfhnd
24twoseven52
425vqnhmrpxthreenine3bkjpvs
8fzqqspdp6
427five9zhbxpbr
1qjnxxkgqhp
eightxzthreelrbgfbvmrpmtgvrfgqmjlshbqv5
onetwo4
three194bfivevknbqxv
1mjhplcvheightlnvn7two
rmptjk5seventhreeonesevenkdxfkvdz
threesix8kpmrvgkpsix
fjfbx6eight3fourninesp
six3nhonebtdzppnxkr
xgtltdtwoninesix62szphpcmtv6
fiveeight9seventhreesevengndgdfv
ninetwosmcbfkvf5hvksvfbr3ssbntlk2
qjrdlmone7tqrzlvfourvfbvtkjxfpkffdpjnine
sevenfqnsnqt89five5
16sevenxtdrtmdzrxhneightwot
8ttreight4eight
three9nine3832four
fivesncggngss8qzfvj
twotwo7sgtcgrmdnr545
6xk7threejmgnqnsevenmczscqxqxfour
twosix4eight
kxm15two844eight
twoeight5sevenfour
five56three
threekdjgcdd6one
clchjoneeightrqgk8bdsdmzm
twog546two6x
5nine4bmhbtbksxreightoneightkg
gjfkbsr1gvtvlnbzqrlfour
one48vpone
eightbcklnvtgvthreeninefivetmgtbjrqnn3
nkclzcvbbq8nineznbtkntgndnine1xrg
lqpfhmrlmg32xtddcfffdbxbjckvlzghpldfgvhfhdhr
fourpkcppxtmqz5
dvb6sixvhdrzzjk
btwokhl99nine
73rkgbsqgz7koneonemlhrbtxc
eighthlqqt4onenine
eight61oneightx
ninebbtcjjlqkthreenfvbcc4three946
ninepqtlffxvclgn7fqdhqbpmkkseveneighttwo
bqzpfxntnkhhfkv98fhrqslfs9four
4ktprrljbthreethreethree3blmddptfour
5fourone
9zhvgleightseven5nbcmscqms2two
6czngsp895
2threekrxmtfrr2ppfrzqkqgfoureightlf
six8eight6seventxgl
1one9
3qpmrtzsvgkjxsevengzdkjkbbdltlrjkznbbkmpb
sclxoneeightfoursfbfm4rbxssmgngfxrvcv
fhctmnpxdrfivemndlr3sdp
twoseven9sixlp
962
tsnsndjtrp27cbtsjdlkrqmtctxvxvnx
5mnine
7fgt6
three1ninertghhbhbcnjdsknine
two8ninefxcqmxdhtnmcmjvtds6fivebnm
7phdp
2hnszbksg
4ninedcnjbcxnrmoneeight1
hnsszlsx7414two4
gmkrn9gcgmffrvbfivethreefive2seven
2bknqdqmrxpfftptwo2vmqffgmzr
dsvf3nv14zklptjnhv
54nchsnpknkx
oneone15
onefvmxnzp575sixpxnpndqf
6vqrzsbbxg3735
twojbldk2hfqqzgone5
onesixlzznvc8nineseven359
6vjbr7
nineone32nmkszsthreefpscxcqtwofour
fv1zpqrxvdlzqmeighthhbbbzt
cskvllzszxzk9
7ninebrlggdzqk7ninelvddrtt1
2fpttpbq6b7ninefour3five
bsslmmmhfxc6
lfjkdzdm9eightkbdhrkpcnzcftlggh
rnineninelmbhfour88onevfzhcmneightwofp
one59ktxrdrhc8six9
86nxnfzzsixgmxxglvfivezmkdvmhjfzone
9threefourfourrbxqpqbtbzxstwo
3pvgtcxrfvthree39bjfivenxvbjone
sixkmngfour3bfive
6threehntsjhjpmqhl345
fourjcnd49fivexqfbj
ksmszhfive7rrphsxxhgm
zqtdztkhveightninepnctbcgqsb6
ninemlvzfcljfkeight1tppxrqtdzp2
twotwoeightnine5frzk3
lfvqtzbsix3
four3bsgft28
sixgksnkrmsix2rbjmfmnfzfiveeight
vrtffvbtcmszfdc2sevenhvpfour8
twopvgzmthree3
xqffprzsrjqkcbsevenczlbc5
4ninefournmvninetwommnrctqjhg
27four
fheightwod4
38zkgvdpgjzqfivenlr7mdxd
ninexxvninesevenfivedpzfgpbv5
xjzgznvfsevensixthree3rgfqhkxbfp5vfrjcdncfkjk
2kcbprjfhns68
bndqgcmnine4qbzfgxmlffive
ffctnxgtwobldqhsix8djfmdnpgmt
fourseven91ninesevenpksgt3
rninesznvtbq5zqmthzrcntskdthree7
8eight7
1hnlnp79
1three3
414ninevqrccrf6lmzqmsjc6
15mpeightxmhxlxjmn
ktdblrmqqxmktvqfour87
1four121three56
ldfdgfqkr22nxtsrsntlsevenxxrcclrhtl6five
eightfourthree8pvlkkbmbrfnfr6
8sixninefiveqcmjhpx
2seveneight85lskjhn7
31c3
hpgmpmjmnk4twothree
cqsxgbj23six11
threesxzmgtvhhheight99xsix
6jlpt6rnsprllqhgbvhtqtjvft217
22onesevenfour4sevenseven
sbnjszzkdleight6three
3twofourdmrsqqtnzgng1two
ldcvxqbsfqpgql9ngsbhfrmszhgvznnnjhnm5
1oneseven28seven
nsvhqtfivemjgcdqpbtwo1nine2
six8dj
zdoneight67fivesevencqlr
xlzrkgjrhmszmkzlkkmrxjdnzrtlzssfpcxkbxvp6jghldhvr9
8sevenbzbnnx
vzrmcfvc9shkxfblfkf
9twoshzmktntwo5nqp
3fourvpdfs3eightword
three24onesixgdzgkspgsjprjgn7eight
4tz2sixsixqbqfsr4twonine
twoq2eight
rccxnvtqnhdlgzqfzcnrmqtjqonesix732
seightwo8ninekndqrdtbfp
cgnmrkcjvbgfmpbpjckhfrzrtnine2sevenfivesxhqk
fourxsncktrjtpnine6ccbgpl
1sixone5vkzxnhgdlbsevendtwo
9jbnineppf7
5foursixlrn5vdgdbvnfhg
fivecrtwo8zbqrvjgpfivenine2
ktvdhgvone6dbrvj5sqbr5
bljtwonesixthree7dzdfrgqrkstwo4xvfmtsbpqjgxsgqcpc
sixgg5
vmrbqdqdrqtwo2onez
onehlgrgndk9ninefsntwo1rnclttm
4tqzjbhdhkm1
nine1kffxzcrn189
sbrxr9ninemqb8
six528mslnf
sdvthree7dfivenine918
3kvzqqkonepmghblzvljnineone47twonesdf
four9sevenfourjhmjjslqgd8
3fkfmgqf6fourbldjfvbhr
eighttxpddhsnzrkplzqc1ninezpvqgnhxzftwo
56lsxmkfpghsqptvhmmmzhcmfdeightsix
one72kdfdrtwothree8
7kpgjhpnthree7
37j2mtwo
seven4mjponefivesdgncqm2gkklsqvthfeight
mgckktt9six4eightwol
4grzfdm499ninetbt
tzmbtv84b
xvzltd75dhpjckmpdrkgglskqrhd5
cgnbhmdlj24
7fivetpbtmone
fivelpmbxvhktzkkonexsxktlzthgbgqkgcj94
5sixhggcbrft
43five7122tqlkhssczsr
eight1seven3
fourtwo134fourmzpxspr9
snxjqnf8ctcnvjknghxpkpbrt2
4qj2xtwoghknine
1zjfivegtwotqgndnineg
fjmmznxkrml7fiveninelfvkqtrg6
8xjlqgqj395nineninefive
dx47sixtwothdphj
fivesevenxgsfb235cvgmspdgg6
fivettfzvfg78dmzzdzcrt
fpnvxp1
seven24
1three8oneeight
7xglzsqrtb8fhgthmgdcdtwo
seven8gdtfgd4fivepdq
twosix1qtttvvstmqbrhh
ptwomnjhlzjjkztwo4kdkgxv
89zkvrhmrhdbmfourzdpss
bfdsvnxflgqxgpzkmrptlvmvlkchjxrt67
9mmhxsevennvvdngznine17twofive
lszmqdmxz87mqbv51
546sevenninencccndnr4
nineeightsqgrpkrqlmzrlxm6hxchcjspnx
8twoccdnbfive
1ks98
twothree6
twoxsixtqdpp6dcclzgfvkv1
16one7scvsgvmcdsixjtzszzrxdzzgv
bjxzdsixkvqgjfzbbjnrtn212jplvrj4
zb2three5cngqfczc1
5fnine99zxvfourjgmn
one634rteightfoureight1
76gvcdfourgtbzdlltzsjnxqg
ltmdrkqqnfour9eightpckdvbhlkvxlpdtp
6two5kgncpnzkdsgnpspb
three8vjninekbbnnfrdstprcmklrgpkfpmbs
ckvvqjqzbpfhf52fivefivefour
7ngxpdqptksix
ftp2eight
8lnmpbqldqstwo
threetwo153644
914
plxjdxghsix17
sevenonesevennine4three2seven
7bvdgpghzhpeight512vxbnfqjctb
xqptzkfive4xqbjzpqfkfspqv5kgqbdtfive
bmltkhjsckhrc7two8nzdpkjjpnfive
nnhppfvlhcmnmrjxhrbtzdflseventwojfprxmfn7
fournfdtjtsbthree54fpzsq
ninenineone5two
8694twomgxxzfxr6
sixeight75seventhreerpchfour8
llv5
skxcbfffgc6sixone
3lhjbzbsg4lsfgpkmcz7vjxzbrshbseven
4onecjlkpqdljd3five5
eighttwo2mvdtvqpnonetvphxsk8
15195one
kdx9nine6qrnqrjxq59
twolhjmbh4fivetwo6pdzbnzt
pbvfour7
tgxrsbk2
hfournineone58sixthree9
hmftwonesix5dhthkcnzqseventhreenineeightnine
9jmmjclsnsdhhj
hljjvctthreefourxsdlvsgtqj1335seven
172
2gbfpjff
five5sevengvddnphnine
sixeightfourssptsqlhzxonetfg2hkmrcpfzcz
94csjjgl42three
4bkxxv6
pgdgoneqmhxfpnfqkrbkjeightbmjjlrnsc5
nineninetwo55fvsxspzt
32jqffghbqvfmjtjone78
93eightfour6eight9eight1
768jrdmfxgxpntx8nhpljjdx
eight33zpvpggklseven4lcqsixthree
fourgrmone4
bcknine9qsevenrjhjeight
5onevqm
578mmqhhljtwo39mfnpmfqthree
fivecmts43eightfdphfivejsx
8vkqvl
8oneone27fourseventwo
jfsixfivethree3cmjtvkzhqkcksmljxxzbjd
three6sixpdvtrnrtffltrs
threeltvvfkcdqjtwo89two8j
5sixeightfivesixjjmknrgd1qpsbpjrffjl
eightrmgfkckxxxsvfclhtgcjthree3
rnfbp8eightjv35eight
2rrssqrfkvmq
dcmghdmg6threetzsdx
qzhfivefivejvbtncm2
gcspvjnsevenqhmkngfivesix15jtpk
bkmlmh5six1sixone2hgtlsix
8rmjshdhm681vc
sixtx32
7fourtwo6
xgrcxeightsevenzgmcllfjqn7
553fivethreefour8nine
one4mxbmct
three6ninetwofour
1bbbxqhhlmj651eightkfdqdgvh9three
4414
mnxsixone92
61six8
eight8mrcpfive1
crsvmfivezbkzkqsix7mxjdgtsqbfvdbnlqtfiveoneightgj
4dmzznftdbqj
seven25
fhlhpvphqvh8one1ffkq
mboneighteightonefive1ninenineninetwonine
82fiveonexc5
97qbhvbqnmxtlpczsx
nhgzzjkx5
hgxlrk9
8twonine9fivetdxmjqppxr
onethree99sevenfourkzvd2
sdqjfvbndzcthree1ftoneightkm
5rcllqcxt
threejdntvhsixsckfpndjzkeightsix3nlgpsvsfhk
94fctwoxmczbkz
4eight35tbqjxglldsevenxmthmmlhsix
three125
3njtlmxtbr4541ninedz7
9ptzpzqrjmxlrmbfbpn9ss4sqcprfmcqg
8dpmkdtvjxzjbddn7pvkxzskddrhcsjvthree7
xxshbfcmf3cfdkeight
six9ninefour472
fourrvxfmjzd4five
1fivesix
twonine3six9
nine8onethreethree
foureighttwotwo7onethree
9prtlnjptoneninefiveninefive
cljqxpthqzdxpmmbvpjljjxhlhsql8kztpthreesevenoneights
9sixtwonetfh
kscmxpcqlthrmthcdhplpnqlq2
fiveqjfvkmnineeightninefourtwo1
one5sixeightfxkshmninethreeeight
4nine4
27
ggtz73sixkgsjrtcxkb1
qvxsgvrpbxqcgpb2fzmcvknkr99seven
6eight8ksgdlxj638fivetcgb
mmvc3fourrvbztjchbmqtxtgfrrqphninefive1
3qzbzxsevenfivegrvtbckqcj
eighttwokzpgl2hrvqlhkthreenine4
8twoeighteightxtsbrseven
nine5mghp7vpnvtpx2c
hkxkmx5qnpjhtdfjfsix2xqqplvm5gmvjm
69cgvzhvgjvl8
threeblzj6three
3bxpmnfbtpk5hcgqkbkqblznxgsdvklmtmqjxsxdcgqvmsprxrrnfchfbnd
1fbrrcjgzzllmcbdrgmrcfsevenh
58onezbp
1bqfkmkk46cctvmstvhvrtwoone
mmeightwo9sbjvleightdsevenseven
9gdsqgflkvonetwo
3lgpmxdgjtzx3two5foursixeightwomb
dbkbeightfive9eighthxngnrbmp
2cpkxggtrdsrh
zsgcdcrlhlqdpone9eight17sixbbtzpmdf
5rskkplgsbl9qqzfrzh67four4
nine856threezgbhrzjcfour
kqnnine87lflxddvtfb9lfjdknvgl8
fsgtwo8zxvnfour7xxfnmqpzhzone4
ddd9sixnsrpqmvvjh2xghxhm
836
nxglt36ljcbvgc16hxcbtqjz
nine19fourhnvh7fgqklf11
jkvsphrpmhbnfl2nhcflhsbks4
7nine2dsgkmrzlrzptfpk672
6vtmztrjdrbk
4vgsshzzsrtwo
ninexfjcxdcnxs7seventhree2
hsvvqcqp97twommjjlclbtdjbxkveightwos
threedlcvvseven3
366xqfbhzfmqknine7
jzrqqfouronehhrmkg4one5
8xcbccrp141kmcsrdlgcdzpcb1nine
59rdrphcgk
fivetwoqmlk22eightfive
qnsphtvfourtwojljxnvgrkk3slv6four
13dfbnfnpsevennqtjthreethree6
4dtncvsix34oneeighthpfouroneightv
xdhqninefive88nine
6hnnl8nineonehm7four
sevenfournine8five2six
vnczlzjqdtmdgsgxch94one3knlxjvqtrjlsx2
twoeight14kkzrsqmgkhjb
jhqrzxrhskngj9ninenjcnbdtjhjtdh
2grrlxnlvthreernfghspmc
two8hjbsevenfive5threenpgtnkftp
d8sixfive1five7
sixthree9
sixspqf2gptcsrvlln9
seven8nine
nqeightwo7svvjqs75qgp3hpvn
qrsixxffsdvvrf74five3
5threeskqgcgprrjmcxksixtwo969
bdbtlvlseven13fh
2two1eightfour7jqdd
three8ninefournffzbnbhkpjqh32
sixqzmhnjttdnine41sevenpcxqkvr
fivenine2fourseven2
sixjjhrjbmvvngqd3three7ninekpneight
11lttrkpcljbbrmponeightbb
frxgkvgrjtsix84
76eightonesix8fivenlfhkfgp
fourpqlrklpnfljvpfkdklkgrjp1ninevfour
1six75tr1
cjdvxhmjvstninejccxrqhb1qkpmnzx8xfpp
8sixfourone6pzlnczvlsmmp
eightrkl37jqlvjjsbrtqsix6
onesmrjvmrtlppm69857seven
4rgsktnbone8sjxjzbrplnmfvkknpxqv
tfteightwonsdffone8sixxdpeightseven
639one2two
smk55
6cjbckhbtmkcgrvmp3
rdnf9
1jjkdspeighthteight37
1gh3dcmhx4sixnphphhbpninevbqhs
vpstbqtdmbvk9jstvtgzrdl
91seventwo
7sdpflkxfzfivethreenine
gvgcrpphhbzghtbcv6
vvnine5ddgzroneone
5four6684one
dmspptjjtwocr2one
sixftzx6fdsfv
5six8twotccbsdnpxg32tsix
mfkdbdfournktdmgqnt4vqkzbzonekdqhbcmfgfiveone
nine2476
8rxd2eightninefourmd
eight1hqfcqs4cpvpsqjfhptwo6six
5z
bqcj948nine6xmgnxmxnn
two1lzdv65bhllvrc
sevenvjkl3ninespltvdszvnfvzpcvrctphvp49
4mzjxftf5eight
fivehbrxcbgjhhxpzfn5lllsknk7six
6n5ttwo166
fqdfhc3
6cph
lzqqvnlkjv3sevenbdssvckmdm
bbdpgpfsevenvzsix87
754sznxkfb4npmjbv4one
twothree984dsxsninefour
kmpjgh71jhfrjgrpbd2d7
27tgptfvcjnk2
onefive497cdcktxcjfivezdrdhczbp
79176threeseven1
csseven1rmlbcpct
1lzlz4tsngmgh
zvxshddzg2eightpxkzfnvdfzeightseven
ninesixnine93seven8
1twofiveeightmfour
six3cmbbbxbqr74flchvgjbr45
7nine31two9dvsghkrjj
five5four
seveneightthreefourseventrgxdqclq2
5dfgddsevenldmzckmvxjmk92fourdcpfgcrpd
ninekbhrflzmgp3lqgxgszmzvndr47zcllcfg
sqsixonefivekfvbfh66p
bssrkdrrgsftlqjdz19
bqcbhfive6
jt3fivemdrnrvngsb1seven5
svdninesbj11eight
2phltdc
1469sevenvksvlthqskfkx9
7fcfivefourdbbvtnjbrc
6seven4
lrmfqtjkzfive69eight8
zmngmr9ninenineninesh
qrxvdxgtfour7one6
eightmjpkgdtrp52shpcrtb
54k7fournine
7fdbshl6874
jfjvrhccqrc1fourcrg96
1sevenncqhkgtzmtncmxhflmfsxfsmsmqh
79fourjmxfdbqnrsr
sjthree6tcbp3x
qvrzmdzfnpkkdcvone2lttcbzhhddbnlnhxgsblhtlvdcpnzjvvqszrthree
ninedfxcz63two515
48sevenvgznbsxzhfgzq19
dxjbtzvtfn3five3
five1gxfcfppffg74
78ncrnhmv766nine
two7qmgeightjhddgnineone
jhttksbpbhzmgglfour7
bloneight3ckvpkxtwozzxr1onethree
mhmppdtvfonecrjzlktlnkpzdbqtvtwo4
179tjchxninethreesix
dpcfnsftnjbhlpcjrc8pbnnhtlrjzmmjk
ninebn16one2crfour
mfivesix8nine1zgpqpr
eight44fourfivegfive
2qhsltnzsfivervftmdm1hthcml
sjoneightvrxctb9sixhkhmfivejm
7threehdctsfqflggzkhpn
seightfourkhpkprrcl6six
fourkdff7
57seven3four1r1gcjtckvn
gmfdzgv7fivesix4nvlq3
thp7
xd5plhtvtgxmgkmhlr5nine
7fourdf
pnkjgdctpm2221four8pvnhxdtrvs
hngtq27nineninesixeight
2fivejpfbsqtx1fourseven
19jpjsbljgz
ninethree5twoeighttwoqponefive
gklmbcj5mstwo8
phbeightwo31jjzltcqzhklm26
jtslpzsxh43
drk7
5dxcsvgqkmz
jnrgv9
nsttntwo6
6qbfrlcfmmnqpmbbtqgdsjqndqfive
2mgdjlpdsixfive6
sevenfivesix6
3nrfvb13onebsrclqnflppzchtp21
dmpgdvfive7
lpvqjzhvjoneghqxnvsixjxflzqj4
onegqzxvnnfnlcffour2
nsglbskbzd2drjzqhnq
nine8seven
rfspthhjbh92rseven1
nine2ctjnb
gzkgvtxjt8twoonefour
tplxnlr8bfjklthree
jqrxrtdvmj8fournmj
6gxvglbcqkdsdcsl
pzjk9ccbmkzmtsf
qdzpknfhbsixczkrqbpfour76sevensbvltnjccjllng
six1zrhqxzfivetwo
xjhmfkvjgxtbgszmpd54jbsscgrninethree
cndbtlq22
fouronesevenzrrv7mhsvjtwokqbfvvrs5
twoonefive59phkxdbndgch5seven
8twoninethreeckdkd5
9four6jfxnfgjvcrszrpsrkhh
993psn4qdldmfnqsix4
42tzxknj
vvzpl8six
8vpchdjxczsvjjqljnmpdeightone2seven
fivenine51sixcdb
dfkkvsone5kjzvqqc
8seven8pxtrdrkqcqhdklgsixzxtjmtblzzcc
ddvzsqnxd9xscsix4rvpsix
75nsvxf9one88
38ninetmmqx2
sxjqgrdthree2vhpgrcdtqeight
63kthndjc
qgbdkfm56two6fourthree5
five79
jpb2seven7one7
four5sbgbpzkjjkjhsvgksgjvfrsbpbdzgd2
xlxpsixninextzlpbn6
jzbvzlflf8twoeight
4vjtr6eightninenjtvt
3twonine3vmrgkjzkpfoursixtbcktpn
six1fourbfhxlntrqfxxztmj4
jrbhfourfour6jspsxkn2eight5four
pcvjmdlnf15cfghpszt9
4lxktgpvvq1sevenone9
twosixhllhbxkbd2three
veightwo3eightvrhseven89onelxvhqxkhm
2sixfkdpcjcgcbgfzlbgblj
457jtmdfdjcnine
three64
pqgjfseven4sevenoneslgeight
91vcptwo1twonineone
seven63fourzjpdmk1
8fivetwofrbtm6f9five
sevenchrtbctkgpnine65fourbhpqnchhlz6
rkdqrbtdbj9bt41hklvlrbrcpx
1three1gmdeighteightqbgsevenhljbpzbdtt
srkbbkfcznine6nine6
8kmzbjzsxtgr9drtbdl
24six412smjfxscst
9cktpshfdr5djvrngchrh176
2mjknvteightmvsdgt8seventhree3mzfk
vhldtk27two
6three2ninepqnine
2fourzbtqnclrtpsix2six94
six6five
one2eightppzmczmgsixnine
15qljk7vchkfcfhkhmbjlkcfour
99four
dhrgkkbrnczkdt3n
89211sevenhnbjbrxtk
663rrbpnrknine911nine
2ntffsix9
4mdpcrvqfoursixqeight
jbjjqkfive3jqnqhvkmtbddrdqxseven
2gvrgdsxptwovfkpgdlmlhz34
rqfrx82
tworzlxeight66eight1five
md1jgknfftttjbjz
tctqctkone37cmpbslgpzh3
3sixsixfpxg
seven6bld42brzxr
6two4seven9zpgb5
nine13
d68
3lxvsgksbbtwo52eight3three8
bpltlnlzc8
seven2five1threedmkg
hpkvhlkhknhjpq9tgpmnndgtlqjx1vkdkvtqhtwodrbr
fourfivesxgpxhdvts7four5
7frgrjkkrb1
2threettvptwobjhmxvpsvmljtvnpkpsvr
9fivelxtcbbn9xgtwohmmhnfhcknc
fourqztfvd6gkhxnstjjxnbl69hzkghsjrd
pspgqcjfcqdgq6xsgxrls
threesix6lrfkmvjdfivetwo9
kdjgpj54
oneeightsevenfive1
one4onefournine64
45hhcpntsthreezl9hjdnnine
17ninenineninegczplbj81
31threetwosevencphdv
5eightvsrzjmdbtqhhqtjfjrhllhbgzgzjzvdhddstxpp4
pgghlbtsevenqxxjbnd14onetwo
3ljgmcxnqgxcrfourfourtworjtwo
eightseventhree59
3nhsccljtzszftnqtfour5
threefourthreeone3five
3pzgzjv7tz
qxgntksdr45tvnxfcjdnn
5seven9threejdzrzdfcpgbnhrrmfkkskg2two
38bjnlcjbeightfivefourqvtbvjsb9nine
nmxqtzlpfngzlsnl9
9eight8five3two1gsknxznbf9
518jkqmprjlqcpdthreefive16
6lzjznblrj3three
f7blbxznlvgk
37jtlrvlzhzronehn
kpckhlcsbeight6
8dfptmfourtwoeightfoursixonefive
ninecdrxlgdkm7nineseventwotbbcgninekbxssd
rrhkgmnsbxbhb2ccnncrbstjfbmnlbxsxbkr
cflrvjtdsevenfivefive3sevensixeightseven
41ninenrznrcpdqhxfglb
gffivesixtqbbmzllvbnjnk2fivesix
eight311seven
52mpqbgktxhs359btkzqdfzvtrzmltxt
onegxnk2rvnmdcmqvqvgkml34
gfzeightwoeightrbvknvpt7
1ct2
zctqtxtgseven66zhslvvdninetwo
54sixbfn8mxfkthf5
9974seven9dclxbmfive
onefour6mjtssrxjjsleight2foureight
818sixsmmzsvbpl9two
rjqnpzlp83qxlj
sxqztv22
8tsvhfszvj
one5mninefour68
three7threeone
sevencnq2jdjvmlh5mqnnnrsqgppkfxjfjsevendrq
8zmxbvgschsqxbk5lltxpseventhreevrhsvdkk
seven2ngbqlxkjl27eighttjprz
ccfskxtnqpqninevczrltkg18hmgjmqt
npgtv4nine5lnqvglvdrxvqmc
kvg4zrtpxnknbone
threeeight1tsdcthree5zxrshttlsmseven9
35448284
6fiveqplkfftsj
pldmrjhzhfiverlgntcckbqzjgth4gfddcrz
6fivesix2
eightksz6m6pneightnvpvvx
1fourbqxtmbvzsfnrxqmvlbfzvdthree
two43sixthree5one
cqf2
twoltseven8three64
6fourprlhcc
fgqoneightsevenqthreebksixgdqt93dm
xjczd3sixseven5
kfzgshnxqnptrckbrt2
fourrrdcl624
kvhfqspcpsxndjqlonesixthree24kdmqvone
8eightfivetwo
onesevenseven5fourlrkkqtfkrmdlsmd
jffvtzkbjnkdtvfsfthree431lrpgmtv
bbvsptrzbone4tfnpgrfourvsix
4seventhreekmxsz335eight
eightbrcv5
two2eightbppsplzgcfournine5seven
fourthree5kcdhqzeighteightkbzszgv8nine
rgxrddnnbv7rkt
8ffmvpcsvfoureightqpnzzjksgchnine9jlgjqb
two9tfvjqsgqsixnine
bzn4two
sqlfeighteight6hjddxzcone2
3fivekmfnqlctddfivelcthnine
twodn8
one5six913lbrcc
foureightmppchbgz8lqbzqbjztwo7cksqxns
zvhzgfpkhkone93nine";
