from typing import List
from itertools import cycle
from functional import seq

vigenere_table = ['abcdefghijklmnopqrstuvwxyz',
                  'bcdefghijklmnopqrstuvwxyza',
                  'cdefghijklmnopqrstuvwxyzab',
                  'defghijklmnopqrstuvwxyzabc',
                  'efghijklmnopqrstuvwxyzabcd',
                  'fghijklmnopqrstuvwxyzabcde',
                  'ghijklmnopqrstuvwxyzabcdef',
                  'hijklmnopqrstuvwxyzabcdefg',
                  'ijklmnopqrstuvwxyzabcdefgh',
                  'jklmnopqrstuvwxyzabcdefghi',
                  'klmnopqrstuvwxyzabcdefghij',
                  'lmnopqrstuvwxyzabcdefghijk',
                  'mnopqrstuvwxyzabcdefghijkl',
                  'nopqrstuvwxyzabcdefghijklm',
                  'opqrstuvwxyzabcdefghijklmn',
                  'pqrstuvwxyzabcdefghijklmno',
                  'qrstuvwxyzabcdefghijklmnop',
                  'rstuvwxyzabcdefghijklmnopq',
                  'stuvwxyzabcdefghijklmnopqr',
                  'tuvwxyzabcdefghijklmnopqrs',
                  'uvwxyzabcdefghijklmnopqrst',
                  'vwxyzabcdefghijklmnopqrstu',
                  'wxyzabcdefghijklmnopqrstuv',
                  'xyzabcdefghijklmnopqrstuvw',
                  'yzabcdefghijklmnopqrstuvwx',
                  'zabcdefghijklmnopqrstuvwxy']


class Caesar:
    # TODO 跳过字母以外的字符
    @classmethod
    def encode(cls, _text: str, offset: int) -> str:
        return seq(iter(_text))\
            .map(lambda c: chr((ord(c)-97+offset) % 26+97))\
            .make_string('')

    @classmethod
    def decode(cls, _text: str, offset: int) -> str:
        return seq(iter(_text))\
            .map(lambda c: chr((ord(c)-97-offset) % 26+97))\
            .make_string('')

    @classmethod
    def shift(cls, text: str, offset: int) -> str:
        return seq(iter(text))\
            .map(lambda c: chr((ord(c)+offset)))\
            .make_string('')


class Keyword:
    alphabet = seq(list('abcdefghijklmnopqrstuvwxyz'))

    @classmethod
    def encode(cls, _text: str, _keyword: str) -> str:
        keyword = seq(list(_keyword.lower()))
        text = seq(iter(_text.lower()))

        table = cls.alphabet\
            .zip(keyword + cls.alphabet.filter(lambda x: x not in keyword))\
            .dict()
        return text.map(lambda x: table[x]).make_string('')

    @classmethod
    def decode(cls, _text: str, _keyword: str) -> str:
        keyword = seq(list(_keyword.lower()))
        text = seq(iter(_text.lower()))

        table = (keyword + cls.alphabet.filter(lambda x: x not in keyword))\
            .zip(cls.alphabet)\
            .dict()
        return text.map(lambda x: table[x]).make_string('')


class Affine:
    @classmethod
    def encode(cls, _text: str, multi: int, offset: int) -> str:
        text = seq(iter(_text.lower()))
        return text\
            .map(lambda x: ord(x)-97)\
            .map(lambda x: x*multi+offset)\
            .map(lambda x: chr(x % 26+97))\
            .make_string('')

    @classmethod
    def decode(cls, _text: str, multi: int, offset: int) -> str:
        text = seq(iter(_text.lower()))
        table = seq.range(26)\
            .map(lambda x: (
                chr(((x*multi+offset) % 26)+97),
                chr(x+97)
            )).dict()
        return text\
            .map(lambda x: table[x])\
            .make_string('')


class Multiliteral:
    # TODO 过滤字符
    alphabet = seq(list('abcdefghiklmnopqrstuvwxyz'))

    @classmethod
    def encode(cls, _text: str, _key: str) -> str:
        key = seq(list(_key))
        text = seq(iter(_text))
        table = cls.alphabet.zip(
            key.cartesian(key).smap(lambda x, y: x+y)
        ).dict()
        return text.map(lambda x: table[x]).make_string('')

    @classmethod
    def decode(cls, _text: str, _key: str) -> str:
        key = seq(list(_key))
        text = seq(iter(_text))
        table = key.cartesian(key)\
            .smap(lambda x, y: x+y)\
            .zip(cls.alphabet)\
            .dict()
        return text.grouped(2)\
            .smap(lambda x, y: table[x+y])\
            .make_string('')


class Vigenere:

    @classmethod
    def encode(cls, _text: str, _key: str) -> str:
        text = seq(list(_text.lower())).map(lambda x: ord(x)-97)
        key = seq(cycle(_key.lower())).map(lambda x: ord(x)-97)
        return text\
            .zip(key)\
            .smap(lambda x, y: vigenere_table[y][x])\
            .make_string('')

    @classmethod
    def decode(cls, _text: str, _key: str) -> str:
        text = seq(list(_text.lower())).map(lambda x: ord(x)-97)
        key = seq(cycle(_key.lower())).map(lambda x: ord(x)-97)
        return text\
            .zip(key)\
            .smap(lambda x, y: vigenere_table[y].find(chr(x+97)))\
            .map(lambda x: vigenere_table[0][x])\
            .make_string('')


class AutokeyCipher:
    @classmethod
    def encode(cls, _text: str, _key: str) -> str:
        # 初始密钥
        key = list(_key.lower())
        # 明文
        text = _text.lower()
        for i in range(len(text)):
            # 使用一位密钥和一位明文查维吉尼亚表获得一位密文，将密文累加到密钥后面
            key.append(vigenere_table[ord(key[i])-97][ord(text[i])-97])
        # 全部密钥去掉开头的初始密钥即为密文
        return ''.join(key[len(_key):])

    @classmethod
    def decode(cls, _text: str, _key: str) -> str:
        text = seq(list(_text.lower())).map(lambda x: ord(x)-97)
        key = seq(list(_key.lower())).map(lambda x: ord(x)-97) + text
        return text\
            .zip(key)\
            .smap(lambda x, y: vigenere_table[y].find(chr(x+97)))\
            .map(lambda x: vigenere_table[0][x])\
            .make_string('')
        '''
        设密文为'iswribzejepqob',密钥为'alice'
        则key变量的值为['a', 'l', 'i', 'c', 'e', 'i', 's', 'w', 'r', 'i', 'b', 'z', 'e', 'j', 'e', 'p', 'q', 'o', 'b']
        return text\ # ['i', 's', 'w', 'r', 'i', 'b', 'z', 'e', 'j', 'e', 'p', 'q', 'o', 'b']
            .zip(key)\ # 将密文与密钥两两配对。如：[('i', 'a'), ('s', 'l'), ('w', 'i'), ('r', 'c'), ('i', 'e'), ('b', 'i'), ('z', 's'), ('e', 'w'), ('j', 'r'), ('e', 'i'), ('p', 'b'), ('q', 'z'), ('o', 'e'), ('b', 'j')]
            .smap(lambda x, y: vigenere_table[y].find(chr(x+97)))\ # 找到密钥在维吉尼亚表中对应的那一行，在这一行中找到密文是第几列
            .map(lambda x: vigenere_table[0][x])\ # 密文所在的列的第一个字母即为明文
            .make_string('')
        '''


class AutokeyPlain:
    @classmethod
    def encode(cls, _text: str, _key: str) -> str:
        # 明文
        text = seq(list(_text.lower())).map(lambda x: ord(x)-97)
        # 密钥（初始密钥+明文）
        key = seq(list(_key.lower())).map(lambda x: ord(x)-97) + text
        # 用维吉尼亚表把明文替换成密文
        return text\
            .zip(key)\
            .smap(lambda x, y: vigenere_table[y][x])\
            .make_string('')

    @classmethod
    def decode(cls, _text: str, _key: str) -> str:
        key = list(_key.lower())
        text = _text.lower()
        for i in range(len(text)):
            # 用密钥找到密文所在的行，在该行中找到密文，获得密文所在的列
            column = vigenere_table[ord(key[i])-97].find(text[i])
            # 这一列的第一个字母就是明文
            key.append(vigenere_table[0][column])
        return ''.join(key[len(_key):])


class Playfair:
    # 用来过滤输入中英文字母以外的字符
    alphabet = seq(list('abcdefghiklmnopqrstuvwxyz'))

    @classmethod
    def encode(cls, _text: str, _key: str, nullchar: str = 'q') -> str:
        key = seq(list(_key.lower()))

        # 在相同的字母间插入nullchar
        # 在最后也放了个nullchar,用来把奇数个字母补成偶数个
        # 如果这个nullchar多出来了会在后面去掉
        text = seq(list(_text.lower().replace('j', 'i')))\
            .sliding(2)\
            .smap(lambda x, y: [x, y] if x != y else [x, nullchar, y])\
            .fold_left([_text[0]], lambda cur, next: cur + next[1:] if next else []) + [nullchar]

        # 生成5x5矩阵
        table = ''.join(key.distinct().filter(lambda x: x != 'j') +
                        cls.alphabet.filter(lambda x: x not in _key)) # 去掉英文字母以外的字符

        # 获得两个字母a,b对应的密文
        def opposite(a, b):
            # 获得a与b的坐标
            pos = seq(a, b)\
                .map(lambda x: table.find(x))\
                .map(lambda x: (x//5, x % 5))
            # 两个字母在同一行
            if pos[0][0] == pos[1][0]:
                return pos.map(lambda x: table[(x[0]*5 + (x[1]+1) % 5)])
            # 两个字母在同一列
            if pos[0][1] == pos[1][1]:
                return pos.map(lambda x: table[(x[0]+1) % 5*5 + x[1]])
            # 两个字母不同行不同列
            return (
                table[pos[0][0]*5 + pos[1][1]],
                table[pos[1][0]*5 + pos[0][1]])

        return text\
            .grouped(2)\
            .filter(lambda x: len(x) == 2)\
            .map(lambda x: opposite(x[0], x[1]))\
            .flatten()\
            .make_string('')

        '''
        return text\
            .grouped(2)\ # 两个字母一组
            .filter(lambda x: len(x) == 2)\ # 如果结尾有多出来的nullchar就去掉
            .map(lambda x: opposite(x[0], x[1]))\ # 换成密文
            .flatten()\ # 把[[a,b],[c,d],[e,f]]变成[a,b,c,d,e,f]
            .make_string('') # 连接成字符串
        '''

    @classmethod
    def decode(cls, _text: str, _key: str) -> str:
        key = seq(list(_key.lower()))
        table = ''.join(key.distinct().filter(lambda x: x != 'j') +
                        cls.alphabet.filter(lambda x: x not in _key))
        text = seq(list(_text.lower())).grouped(2)

        def opposite(a, b):
            pos = seq(a, b)\
                .map(lambda x: table.find(x))\
                .map(lambda x: (x//5, x % 5))
            if pos[0][0] == pos[1][0]:
                return pos.map(lambda x: table[(x[0]*5 + (x[1]-1) % 5)])
            if pos[0][1] == pos[1][1]:
                return pos.map(lambda x: table[(x[0]-1) % 5*5 + x[1]])
            return (
                table[pos[0][0]*5 + pos[1][1]],
                table[pos[1][0]*5 + pos[0][1]])
        return text.smap(opposite).flatten().make_string('')


class Permutation:
    @classmethod
    def encode(cls, _text: str, _rule: List[int], nullchar: str = 'x') -> str:
        # _rule是类似[1, 3, 0, 2]的东西，充当密钥
        # 把密文放进矩阵，按1,3,0,2的顺序从矩阵的每一行中拿字母可以获得明文

        # 用nullchar把明文补齐到len(_rule)的整数倍
        text = seq(list(_text + nullchar*(len(_rule) - len(_text) % len(_rule))))

        # 这段代码用于控制后面按什么顺序从每一行中拿出明文连到一起
        # 例：如果按2,0,3,1的顺序把明文放进矩阵，按1,3,0,2的顺序从矩阵中拿字母即可得到明文
        #
        # 第一行：_rule:[1, 3, 0, 2]
        # 第二行：将rule的每个元素替换为tuple(元素,这个元素是第几个)。例：[(1,0),(3,1),(0,2),(2,3)]
        # 第三行：按每个元组的第一个元素排序,得到[(0,2),(1,0),(2,3),(3,1)]
        # 第四行：取每个元组的第二个元素，得[2, 0, 3, 1]
        # 第五行：将结果转换成python自带的列表
        rule = seq(_rule)\
            .zip_with_index()\
            .sorted(lambda x: x[0])\
            .map(lambda x: x[1])\
            .list()

        # 第一行：明文
        # 第二行：将明文分成len(rule)个字母一组，即分成了矩阵中的每一行一组
        # 第三行：给每一行改变顺序
        # 第四、五行：把每一行连接成字符串
        return text\
            .grouped(len(rule))\
            .map(lambda x: [x[i] for i in rule])\
            .flatten()\
            .make_string('')

    @classmethod
    def decode(cls, _text: str, _rule: List[int]) -> str:
        '''
        rule = seq(_rule)\
            .zip_with_index()\
            .sorted(lambda x: x[1])\
            .map(lambda x: x[0])\
            .list()
            '''
        return seq(list(_text))\
            .grouped(len(_rule))\
            .map(lambda x: [x[i] for i in _rule])\
            .flatten()\
            .make_string('')


class ColumnPermutation:
    @classmethod
    def encode(cls, _text: str, _key: str, nullchar: str = 'x') -> str:
        # _key：密钥，这里设密钥为'teem'
        # 按密钥中每个字母在字母表中出现的顺序给它编号，得到3,0,1,2
        # 把明文放进宽为len(_key)的矩阵，依次取出第3、0、1、2列，把每列都连成一个字符串，再把4个字符串连到一起即为密文

        # 把明文补齐到密钥长度的整数倍
        width = len(_key)
        m = len(_text) % width
        if m != 0:
            _text += nullchar*(width-m)

        key = seq(list(_key))

        # 把明文按行分组
        text = seq(list(_text.lower())).grouped(width)

        # 第1～6行用来把['t', 'e', 'e', 'm']变成[3, 0, 1, 2]
        # 第7行依次从明文矩阵中拿出第3、0、1、2列
        # 第8～9行将打乱后的列连到一起
        return key\
            .zip_with_index()\
            .sorted(lambda x: x[0])\
            .zip_with_index()\
            .sorted(lambda x: x[0][1])\
            .map(lambda x: x[1])\
            .map(lambda x: text.map(lambda y: y[x]))\
            .flatten()\
            .make_string('')
        '''
        key                             # t         e         e         m
            .zip_with_index()           # (t,0)     (e,1)     (e,2)     (m,3)
            .sorted(lambda x: x[0])     # (e,1)     (e,2)     (m,3)     (t,0)
            .zip_with_index()           # ((e,1),0) ((e,2),1) ((m,3),2) ((t,0),3)
            .sorted(lambda x: x[0][1])  # ((t,0),3) ((e,1),0) ((e,2),1) ((m,3),2)
            .map(lambda x: x[1])        # 3         0          1        2
        '''
    @classmethod
    def decode(cls, _text: str, _key: str) -> str:
        width = len(_key)
        height = len(_text)//width

        # 按列分组
        text = seq(list(_text.lower())).grouped(height).list()

        plain = seq(list(_key))\
            .zip_with_index()\
            .sorted(lambda x: x[0])\
            .map(lambda x: x[1])\
            .map(lambda x: text[x])\
            .list()
        '''
        plain = seq(list(_key))\        # t         e         e         m
            .zip_with_index()\          # (t,0)     (e,1)     (e,2)     (m,3)
            .sorted(lambda x: x[0])\    # (e,1)     (e,2)     (m,3)     (t,0)
            .map(lambda x: x[1])\       # 1         2         3         0
            .map(lambda x: text[x])\ # (密文的)第2列 第3列      第4列       第1列
            .list()
        '''

        # 把4列明文连起来
        ret = ''
        for i in range(height):
            for j in range(width):
                ret += plain[j][i]
        return ret

    @classmethod
    def double_encode(cls, _text: str, _key1: str, _key2: str, nullchar: str = 'x') -> str:
        return cls.encode(
            cls.encode(_text, _key1, nullchar),
            _key2
        )

    @classmethod
    def double_decode(cls, _text: str, _key1: str, _key2: str) -> str:
        return cls.decode(
            cls.decode(_text, _key2),
            _key1
        )


if __name__ == '__main__':
    assert Caesar.encode('abcdef', 28) == 'cdefgh'
    assert Caesar.decode('cdefgh', 28) == 'abcdef'
    assert Keyword.encode('abcdef', 'cfg') == 'cfgabd'
    assert Keyword.decode('cfgabd', 'cfg') == 'abcdef'
    assert Affine.encode('abcdef', 7, 12) == 'mtahov'
    assert Affine.decode('mtahov', 7, 12) == 'abcdef'
    assert Multiliteral.encode('abcdef', 'btrfs') == 'bbbtbrbfbstb'
    assert Multiliteral.decode('bbbtbrbfbstb', 'btrfs') == 'abcdef'
    assert Vigenere.encode('abcdef', 'amr') == 'antdqw'
    assert Vigenere.decode('antdqw', 'amr') == 'abcdef'
    assert AutokeyCipher.encode('ihopethisworks', 'alice') == 'iswribzejepqob'
    assert AutokeyCipher.decode('iswribzejepqob', 'alice') == 'ihopethisworks'
    assert AutokeyPlain.encode('ihopethisworks', 'alice') == 'iswribowhahysk'
    assert AutokeyPlain.decode('iswribowhahysk', 'alice') == 'ihopethisworks'
    assert Playfair.encode('nexttimejaytrysomethingdifferent', 'telegram') == \
        'hrvlolhblhcvetgypaleikkrbniohttlfr'
    assert Playfair.decode('hrvlolhblhcvetgypaleikkrbniohttlfr', 'telegram') == \
        'nextqtimeiaytrysomethingdifqferent'
    assert Permutation.encode(
        'codesandciphersarefun', [1, 3, 0, 2]) == 'dceonsdapchisearfruexnxx'
    assert Permutation.decode(
        'dceonsdapchisearfruexnxx', [1, 3, 0, 2]) == 'codesandciphersarefunxxx'
    assert ColumnPermutation.encode(
        'encryptionalgorithms', 'teem') == 'riliseyogtnpnohctarm'
    assert ColumnPermutation.decode(
        'riliseyogtnpnohctarm', 'teem') == 'encryptionalgorithms'
    assert ColumnPermutation.double_encode(
        'usingaciphertwicemayimprovethestrenthofthecipheroritmaynot',
        'what',
        'next'
    ) == 'irtnhvetthpianinyttamoapohoisectrsmergihmeecxrtixwehuerocpfy'
    assert ColumnPermutation.double_decode(
        'irtnhvetthpianinyttamoapohoisectrsmergihmeecxrtixwehuerocpfy',
        'what',
        'next'
    ) == 'usingaciphertwicemayimprovethestrenthofthecipheroritmaynotxx'
