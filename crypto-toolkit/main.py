import typing

import PySide2
from PySide2.QtCore import Slot
from PySide2.QtWidgets import (QApplication, QDialog, QFileDialog, QLabel, QLineEdit,
                               QMainWindow, QWidget, QWizard)

import classic
import RC4
import DES
import RSA_implement
import MD5
import ECC
from bare_mainwindow import Ui_MainWindow
from ui_text_encryption_widget import Ui_TextEncryptionWidget
from ui_asymmetric_encryption_widget import Ui_AsymmetricEncryptionWidget
from ui_advance_dialog import Ui_AdvanceDialog
from ui_ECC_keygen_wizard import Ui_ECCKeygenWizard
from ui_md5 import Ui_MD5Widget


class AdvanceDialog(QDialog):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.ui = Ui_AdvanceDialog()
        self.ui.setupUi(self)


class ECCKeygenWizard(QWizard):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.ui = Ui_ECCKeygenWizard()
        self.ui.setupUi(self)


class ClassicCipherWidget(QWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.ui = Ui_TextEncryptionWidget()
        self.ui.setupUi(self)
        self.ui.encryptPushButton.clicked.connect(self.handle_encrypt)
        self.ui.decryptPushButton.clicked.connect(self.handle_decrypt)
        self.layout().setMargin(0)
        self.encrypt_func = print
        self.decrypt_func = print

    # def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget]) -> None:
    #     super().__init__(parent=parent)
    #     self.ui = Ui_ClassicCipherGroupBox()
    #     self.ui.setupUi(self)
    #     self.ui.encryptPushButton.clicked.connect(self.handle_encrypt)
    #     self.ui.decryptPushButton.clicked.connect(self.handle_decrypt)
    #     self.encrypt_func = print
    #     self.decrypt_func = print

    @Slot()
    def handle_encrypt(self):
        plaintext = self.ui.plainTextEdit.toPlainText()
        self.ui.plainTextEdit.setPlainText(plaintext)
        key = self.ui.keyLineEdit.text()
        ciphertext = self.encrypt_func(plaintext, key)
        self.ui.cipherTextEdit.setPlainText(ciphertext)

    @Slot()
    def handle_decrypt(self):
        ciphertext = self.ui.cipherTextEdit.toPlainText()
        self.ui.cipherTextEdit.setPlainText(ciphertext)
        key = self.ui.keyLineEdit.text()
        plaintext = self.decrypt_func(ciphertext, key)
        self.ui.plainTextEdit.setPlainText(plaintext)


class ModernCipherWidget(QWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.ui = Ui_AsymmetricEncryptionWidget()
        self.ui.setupUi(self)

        # ????????????????????????????????????
        self.ui.plainFileLabel.setVisible(False)
        self.ui.cipherFileLabel.setVisible(False)

        # ???????????????????????????widget
        self.ui.privateKeyWidget.hide()
        self.ui.publicKeyWidget.hide()
        self.ui.advancesButton.hide()

        # 5?????????
        self.ui.encryptButton.clicked.connect(self.handle_encrypt)
        self.ui.decryptButton.clicked.connect(self.handle_decrypt)
        self.ui.plainChooseFileButton.clicked.connect(
            self.handle_choose_plainfile)
        self.ui.cipherChooseFileButton.clicked.connect(
            self.handle_choose_cipherfile)
        self.ui.keyImportButton.clicked.connect(self.handle_choose_keyfile)

        self.encrypt_func = lambda x, y: x
        self.decrypt_func = lambda x, y: x

    @Slot()
    def handle_encrypt(self):
        '''??????tabWidget?????????????????????????????????????????????????????????\n
        keyLineEdit????????????????????????????????????keyfilename'''

        # ????????????
        src_mode = self.ui.plainTabWidget.currentIndex()
        if src_mode == 0:
            src_data = self.ui.plainTextEdit.toPlainText()
            self.ui.plainTextEdit.setPlainText(src_data)
            src_data = src_data.encode()
        elif src_mode == 1:
            src_data = open(self.plain_filename, 'rb').read()

        # ????????????
        if self.ui.keyLineEdit.text():
            keydata = self.ui.keyLineEdit.text().encode()
        else:
            keydata = open(self.keyfilename, 'rb').read()

        # ??????
        dest_data: bytes = self.encrypt_func(src_data, keydata)

        # ????????????
        dest_mode = self.ui.cipherTabWidget.currentIndex()
        if dest_mode == 0:
            self.ui.cipherTextEdit.setPlainText(
                dest_data.decode(errors='replace'))
        elif dest_mode == 1:
            open(self.cipher_filename, 'wb').write(dest_data)

    @Slot()
    def handle_decrypt(self):

        # ????????????
        src_mode = self.ui.cipherTabWidget.currentIndex()
        if src_mode == 0:
            src_data = self.ui.cipherTextEdit.toPlainText()
            self.ui.cipherTextEdit.setPlainText(src_data)
            src_data = src_data.encode()
        elif src_mode == 1:
            src_data = open(self.cipher_filename, 'rb').read()

        # ????????????
        if self.ui.keyLineEdit.text():
            keydata = self.ui.keyLineEdit.text().encode()
        else:
            keydata = open(self.keyfilename, 'rb').read()

        # ??????
        dest_data: bytes = self.decrypt_func(src_data, keydata)

        # ????????????
        dest_mode = self.ui.plainTabWidget.currentIndex()
        if dest_mode == 0:
            self.ui.plainTextEdit.setPlainText(
                dest_data.decode(errors='replace'))
        elif dest_mode == 1:
            open(self.plain_filename, 'wb').write(dest_data)

    @Slot()
    def handle_choose_plainfile(self):
        filename: str = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            text = f'???????????????{filename}'
            self.ui.plainFileLabel.setText(text)
            self.ui.plainFileLabel.setVisible(True)
            self.plain_filename = filename
            #self.plaindata = open(filename, 'b').read()

    @Slot()
    def handle_choose_cipherfile(self):
        filename: str = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            text = f'???????????????{filename}'
            self.ui.cipherFileLabel.setText(text)
            self.ui.cipherFileLabel.setVisible(True)
            self.cipher_filename = filename
            #self.cipherdata = open(filename, 'b').read()

    @Slot()
    def handle_choose_keyfile(self):
        '''?????????????????????????????????keyfilename?????????keyLineEdit'''
        filename = QFileDialog.getOpenFileName(self, '???????????????????????????')[0]
        if filename:
            self.keyfilename = filename
            self.ui.keyLineEdit.setText('')
            self.ui.keyLineEdit.setPlaceholderText(f'???????????????{filename}')


class CaesarWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        self.ui.keyLabel.setText('??????')
        self.ui.keyLineEdit.setPlaceholderText('??????')
        # self.setTitle('????????????')
        self.encrypt_func = lambda x, y: classic.Caesar.encode(x, int(y))
        self.decrypt_func = lambda x, y: classic.Caesar.decode(x, int(y))


class KeywordWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        self.encrypt_func = classic.Keyword.encode
        self.decrypt_func = classic.Keyword.decode
        # self.setTitle('??????????????????')


class AffineWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        # self.setTitle('?????????????????????')
        self.ui.keyLabel.setText('a')
        self.ui.keyLayout.addWidget(QLabel('b'))
        self.ui.key2LineEdit = QLineEdit()
        self.ui.keyLayout.addWidget(self.ui.key2LineEdit)
        self.setTabOrder(self.ui.keyLineEdit, self.ui.key2LineEdit)

    @Slot()
    def handle_encrypt(self):
        plaintext = self.ui.plainTextEdit.toPlainText()
        self.ui.plainTextEdit.setPlainText(plaintext)
        a = int(self.ui.keyLineEdit.text())
        b = int(self.ui.key2LineEdit.text())
        ciphertext = classic.Affine.encode(
            plaintext, a, b)
        self.ui.cipherTextEdit.setPlainText(ciphertext)

    @Slot()
    def handle_decrypt(self):
        ciphertext = self.ui.cipherTextEdit.toPlainText()
        self.ui.cipherTextEdit.setPlainText(ciphertext)
        a = int(self.ui.keyLineEdit.text())
        b = int(self.ui.key2LineEdit.text())
        plaintext = classic.Affine.decode(
            ciphertext, a, b)
        self.ui.plainTextEdit.setPlainText(plaintext)


class MultiliteralWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        self.encrypt_func = classic.Multiliteral.encode
        self.decrypt_func = classic.Multiliteral.decode
        # self.setTitle('??????????????????')
        self.ui.keyLineEdit.setPlaceholderText('5?????????')


class VigenereWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        self.encrypt_func = classic.Vigenere.encode
        self.decrypt_func = classic.Vigenere.decode
        # self.setTitle('?????????????????????')


class AutokeyCipherWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        self.encrypt_func = classic.AutokeyCipher.encode
        self.decrypt_func = classic.AutokeyCipher.decode
        # self.setTitle('?????????????????????????????????')


class AutokeyPlainWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        self.encrypt_func = classic.AutokeyPlain.encode
        self.decrypt_func = classic.AutokeyPlain.decode
        # self.setTitle('?????????????????????????????????')


class PlayfairWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        self.encrypt_func = classic.Playfair.encode
        self.decrypt_func = classic.Playfair.decode
        # self.setTitle('Playfair?????????')


class PermutationWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        # self.setTitle('?????????????????????')
        self.ui.keyLineEdit.setPlaceholderText('??????????????????????????????1,3,0,2???')
        self.encrypt_func = lambda text, key:\
            classic.Permutation.encode(
                text, list(map(lambda x: int(x), key.split(','))))
        self.decrypt_func = lambda text, key:\
            classic.Permutation.decode(
                text, list(map(lambda i: int(i), key.split(','))))


class ColumnPermutationWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        self.encrypt_func = classic.ColumnPermutation.encode
        self.decrypt_func = classic.ColumnPermutation.decode
        # self.setTitle('??????????????????')


class DoubleColumnPermutationWidget(ClassicCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent)
        # self.setTitle('?????????????????????')
        self.ui.keyLabel.setText('?????????')
        self.ui.keyLayout.addWidget(QLabel('?????????'))
        self.ui.key2LineEdit = QLineEdit()
        self.ui.keyLayout.addWidget(self.ui.key2LineEdit)
        self.setTabOrder(self.ui.keyLineEdit, self.ui.key2LineEdit)

    @Slot()
    def handle_encrypt(self):
        plaintext = self.ui.plainTextEdit.toPlainText()
        self.ui.plainTextEdit.setPlainText(plaintext)
        key1 = self.ui.keyLineEdit.text()
        key2 = self.ui.key2LineEdit.text()
        ciphertext = classic.ColumnPermutation.double_encode(
            plaintext, key1, key2)
        self.ui.cipherTextEdit.setPlainText(ciphertext)

    @Slot()
    def handle_decrypt(self):
        ciphertext = self.ui.cipherTextEdit.toPlainText()
        self.ui.cipherTextEdit.setPlainText(ciphertext)
        key1 = self.ui.keyLineEdit.text()
        key2 = self.ui.key2LineEdit.text()
        plaintext = classic.ColumnPermutation.double_decode(
            ciphertext, key1, key2)
        self.ui.plainTextEdit.setPlainText(plaintext)


class RC4Widget(ModernCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.encrypt_func = RC4.encrypt
        self.decrypt_func = RC4.decrypt


class DESWidget(ModernCipherWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.encrypt_func = DES.des_encrypt
        self.encrypt_func = lambda x, y: DES.encrypt_binstr(x, y).encode()
        self.decrypt_func = lambda x, y: DES.decrypt_binstr(x, y)
        #self.decrypt_func = DES.decrypt_binstr
        self.ui.plainTextEdit.setPlaceholderText(
            '?????????????????????????????????8???????????????')
        self.ui.cipherTextEdit.setPlaceholderText(
            '???????????????????????????64?????????????????????')


class RSAWidget(QWidget):
    '''TODO:?????????????????????ECC???RSA??????????????????????????????widget???\n
    ???????????????????????????'''

    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.ui = Ui_AsymmetricEncryptionWidget()
        self.ui.setupUi(self)
        self.ui.plainFileLabel.hide()
        self.ui.cipherFileLabel.hide()

        # ????????????????????????widget
        self.ui.keyWidget.hide()

        # 7?????????
        self.ui.encryptButton.clicked.connect(self.handle_encrypt)
        self.ui.decryptButton.clicked.connect(self.handle_decrypt)
        self.ui.importPublicKeyButton.clicked.connect(
            self.handle_choose_publickeyfile)
        self.ui.importPrivateKeyButton.clicked.connect(
            self.handle_choose_privatekeyfile)
        self.ui.plainChooseFileButton.clicked.connect(
            self.handle_choose_plainfile)
        self.ui.cipherChooseFileButton.clicked.connect(
            self.handle_choose_cipherfile)
        self.ui.advancesButton.clicked.connect(self.handle_advance)

        # self.advancesWidget.setGeometry(
        #     QApplication.desktop().width()//2-200,
        #     QApplication.desktop().height()//2-150,
        #     400, 300)

        self.advanceDialog = AdvanceDialog()
        self.advanceDialog.ui.buttonBox.accepted.connect(
            self.handle_save_settings)
        self.advanceDialog.ui.generateKeyButton.clicked.connect(
            self.handle_generate_key)

        self.e = 65537
        self.advanceDialog.ui.eLineEdit.setText(str(self.e))

        self.encrypt_func = lambda text, key:\
            str(RSA_implement.encrypt(text, int(key), self.e)).encode()
        self.decrypt_func = lambda text, key:\
            RSA_implement.decrypt(
                int(text), int(key), int(self.ui.publicKeyLineEdit.text()))

    @Slot()
    def handle_encrypt(self):
        '''??????tabWidget?????????????????????????????????????????????????????????\n
        keyLineEdit????????????????????????????????????keyfilename'''

        # ????????????
        src_mode = self.ui.plainTabWidget.currentIndex()
        if src_mode == 0:
            src_data = self.ui.plainTextEdit.toPlainText()
            self.ui.plainTextEdit.setPlainText(src_data)
            src_data = src_data.encode()
        elif src_mode == 1:
            src_data = open(self.plain_filename, 'rb').read()

        # ????????????
        if self.ui.publicKeyLineEdit.text():
            keydata = self.ui.publicKeyLineEdit.text().encode()
        else:
            keydata = open(self.publickey_filename, 'rb').read()

        # ??????
        dest_data: bytes = self.encrypt_func(src_data, keydata)

        # ????????????
        dest_mode = self.ui.cipherTabWidget.currentIndex()
        if dest_mode == 0:
            self.ui.cipherTextEdit.setPlainText(
                dest_data.decode(errors='replace'))
        elif dest_mode == 1:
            open(self.cipher_filename, 'wb').write(dest_data)

    @Slot()
    def handle_decrypt(self):

        # ????????????
        src_mode = self.ui.cipherTabWidget.currentIndex()
        if src_mode == 0:
            src_data = self.ui.cipherTextEdit.toPlainText()
            self.ui.cipherTextEdit.setPlainText(src_data)
            src_data = src_data.encode()
        elif src_mode == 1:
            src_data = open(self.cipher_filename, 'rb').read()

        # ????????????
        if self.ui.privateKeyLineEdit.text():
            keydata = self.ui.privateKeyLineEdit.text().encode()
        else:
            keydata = open(self.privatekey_filename, 'rb').read()

        # ??????
        dest_data: bytes = self.decrypt_func(src_data, keydata)

        # ????????????
        dest_mode = self.ui.plainTabWidget.currentIndex()
        if dest_mode == 0:
            self.ui.plainTextEdit.setPlainText(
                dest_data.decode(errors='replace'))
        elif dest_mode == 1:
            open(self.plain_filename, 'wb').write(dest_data)

    @Slot()
    def handle_choose_plainfile(self):
        filename: str = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            text = f'???????????????{filename}'
            self.ui.plainFileLabel.setText(text)
            self.ui.plainFileLabel.setVisible(True)
            self.plain_filename = filename
            #self.plaindata = open(filename, 'b').read()

    @Slot()
    def handle_choose_cipherfile(self):
        filename: str = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            text = f'???????????????{filename}'
            self.ui.cipherFileLabel.setText(text)
            self.ui.cipherFileLabel.setVisible(True)
            self.cipher_filename = filename
            #self.cipherdata = open(filename, 'b').read()

    @Slot()
    def handle_choose_privatekeyfile(self):
        '''?????????????????????????????????keyfilename?????????keyLineEdit'''
        filename = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            self.privatekey_filename = filename
            self.ui.privateKeyLineEdit.setText('')
            self.ui.privateKeyLineEdit.setPlaceholderText(f'???????????????{filename}')

    @Slot()
    def handle_choose_publickeyfile(self):
        '''?????????????????????????????????keyfilename?????????keyLineEdit'''
        filename = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            self.publickey_filename = filename
            self.ui.publicKeyLineEdit.setText('')
            self.ui.publicKeyLineEdit.setPlaceholderText(f'???????????????{filename}')

    @Slot()
    def handle_advance(self):
        self.advanceDialog.show()

    @Slot()
    def handle_save_settings(self):
        e = self.advanceDialog.ui.eLineEdit.text()
        if e:
            self.e = int(self.advanceDialog.ui.eLineEdit.text())

    @Slot()
    def handle_generate_key(self):
        keys = RSA_implement.keygen(self.e)
        self.advanceDialog.ui.publicKeyLineEdit.setText(str(keys[0]))
        self.advanceDialog.ui.privateKeyLineEdit.setText(str(keys[1]))


class ECCWidget(QWidget):
    '''TODO:?????????????????????ECC???RSA??????????????????????????????widget???\n
    ???????????????????????????'''

    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.ui = Ui_AsymmetricEncryptionWidget()
        self.ui.setupUi(self)
        self.ui.plainFileLabel.hide()
        self.ui.cipherFileLabel.hide()

        # ????????????????????????widget
        self.ui.keyWidget.hide()

        # 7?????????
        self.ui.encryptButton.clicked.connect(self.handle_encrypt)
        self.ui.decryptButton.clicked.connect(self.handle_decrypt)
        self.ui.importPublicKeyButton.clicked.connect(
            self.handle_choose_publickeyfile)
        self.ui.importPrivateKeyButton.clicked.connect(
            self.handle_choose_privatekeyfile)
        self.ui.plainChooseFileButton.clicked.connect(
            self.handle_choose_plainfile)
        self.ui.cipherChooseFileButton.clicked.connect(
            self.handle_choose_cipherfile)
        self.ui.advancesButton.clicked.connect(self.handle_keygen)

        self.ui.advancesButton.setText('???????????????...')

        # self.advancesWidget.setGeometry(
        #     QApplication.desktop().width()//2-200,
        #     QApplication.desktop().height()//2-150,
        #     400, 300)

        self.keygenWizard = ECCKeygenWizard()
        self.keygenWizard.currentIdChanged.connect(self.handle_wizard)

        self.encrypt_func = lambda text, key:\
            str(RSA_implement.encrypt(text, int(key), self.e)).encode()
        self.decrypt_func = lambda text, key:\
            RSA_implement.decrypt(
                int(text), int(key), int(self.ui.publicKeyLineEdit.text()))

    @Slot()
    def handle_encrypt(self):
        '''??????tabWidget?????????????????????????????????????????????????????????\n
        keyLineEdit????????????????????????????????????keyfilename'''

        # ????????????
        src_mode = self.ui.plainTabWidget.currentIndex()
        if src_mode == 0:
            src_data = self.ui.plainTextEdit.toPlainText()
            self.ui.plainTextEdit.setPlainText(src_data)
            src_data = src_data.encode()
        elif src_mode == 1:
            src_data = open(self.plain_filename, 'rb').read()

        # ????????????
        if self.ui.publicKeyLineEdit.text():
            keydata = self.ui.publicKeyLineEdit.text().encode()
        else:
            keydata = open(self.publickey_filename, 'rb').read()

        # ??????
        #dest_data: bytes = self.encrypt_func(src_data, keydata)
        res = str(ECC.encrypt(src_data, self.K[0], self.K[1], self.gx,
                              self.gy, self.a, self.p, self.rank, self.k))

        # ????????????
        dest_mode = self.ui.cipherTabWidget.currentIndex()
        if dest_mode == 0:
            self.ui.cipherTextEdit.setPlainText(res)
        elif dest_mode == 1:
            open(self.cipher_filename, 'w').write(res)

    @Slot()
    def handle_decrypt(self):

        # ????????????
        src_mode = self.ui.cipherTabWidget.currentIndex()
        if src_mode == 0:
            src_data = self.ui.cipherTextEdit.toPlainText()
            self.ui.cipherTextEdit.setPlainText(src_data)
        elif src_mode == 1:
            src_data = open(self.cipher_filename, 'r').read()

        # ????????????
        if self.ui.privateKeyLineEdit.text():
            keydata = self.ui.privateKeyLineEdit.text()
        else:
            keydata = open(self.privatekey_filename, 'rb')

        # ??????
        #dest_data: bytes = self.decrypt_func(src_data, keydata)
        dest = ECC.decrypt(eval(src_data), self.k, self.a, self.p)

        # ????????????
        dest_mode = self.ui.plainTabWidget.currentIndex()
        if dest_mode == 0:
            self.ui.plainTextEdit.setPlainText(dest.decode())
        elif dest_mode == 1:
            open(self.plain_filename, 'w').write(dest)

    @Slot()
    def handle_choose_plainfile(self):
        filename: str = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            text = f'???????????????{filename}'
            self.ui.plainFileLabel.setText(text)
            self.ui.plainFileLabel.setVisible(True)
            self.plain_filename = filename
            #self.plaindata = open(filename, 'b').read()

    @Slot()
    def handle_choose_cipherfile(self):
        filename: str = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            text = f'???????????????{filename}'
            self.ui.cipherFileLabel.setText(text)
            self.ui.cipherFileLabel.setVisible(True)
            self.cipher_filename = filename
            #self.cipherdata = open(filename, 'b').read()

    @Slot()
    def handle_choose_privatekeyfile(self):
        '''?????????????????????????????????keyfilename?????????keyLineEdit'''
        filename = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            self.privatekey_filename = filename
            self.ui.privateKeyLineEdit.setText('')
            self.ui.privateKeyLineEdit.setPlaceholderText(f'???????????????{filename}')

    @Slot()
    def handle_choose_publickeyfile(self):
        '''?????????????????????????????????keyfilename?????????keyLineEdit'''
        filename = QFileDialog.getOpenFileName(self, '??????????????????')[0]
        if filename:
            self.publickey_filename = filename
            self.ui.publicKeyLineEdit.setText('')
            self.ui.publicKeyLineEdit.setPlaceholderText(f'???????????????{filename}')

    @Slot()
    def handle_keygen(self):
        self.keygenWizard.show()

    @Slot(int)
    def handle_wizard(self, cur_id: int):
        if cur_id == 1:
            a = int(self.keygenWizard.ui.aLineEdit.text())
            b = int(self.keygenWizard.ui.bLineEdit.text())
            p = int(self.keygenWizard.ui.pLineEdit.text())
            self.a = a
            self.b = b
            self.p = p

            table = ECC.get_graph(self.a, self.b, self.p)
            tablestr = ''

            for i in range(p):
                temp = p-1-i
                if temp >= 10:
                    tablestr += f'{temp} '
                else:
                    tablestr += f'{temp}  '
                for j in range(p):
                    tablestr += f'{table[j][temp]}  '
                tablestr += '\n'

            tablestr += '   '
            for i in range(p):
                if i >= 10:
                    tablestr += f'{i} '
                else:
                    tablestr += f'{i}  '
            tablestr += '\n'

            self.keygenWizard.ui.axisLabel.setText(tablestr)

        elif cur_id == 2:
            self.gx = int(self.keygenWizard.ui.gxLineEdit.text())
            self.gy = int(self.keygenWizard.ui.gyLineEdit.text())

            self.rank = ECC.get_rank(self.gx, self.gy, self.a, self.b, self.p)
            self.keygenWizard.ui.privateKeyHintLabel.setText(
                f'?????????????????????{self.rank}????????????')
        elif cur_id == 3:
            self.k = int(self.keygenWizard.ui.inputPrivateKeyLineEdit.text())

            self.K = ECC.get_kG(self.gx, self.gy, self.k, self.a, self.p)

            self.keygenWizard.ui.publicKeyLineEdit.setText(str(self.K))
            self.keygenWizard.ui.privateKeyLineEdit.setText(
                str(self.k))

    @Slot()
    def handle_save_settings(self):
        e = self.advanceDialog.ui.eLineEdit.text()
        if e:
            self.e = int(self.advanceDialog.ui.eLineEdit.text())

    @Slot()
    def handle_generate_key(self):
        keys = RSA_implement.keygen(self.e)
        self.advanceDialog.ui.publicKeyLineEdit.setText(str(keys[0]))
        self.advanceDialog.ui.privateKeyLineEdit.setText(str(keys[1]))


class MD5Widget(QWidget):
    def __init__(self, parent: typing.Optional[PySide2.QtWidgets.QWidget] = None) -> None:
        super().__init__(parent=parent)
        self.ui = Ui_MD5Widget()
        self.ui.setupUi(self)
        self.ui.lineEdit.hide()
        self.ui.pushButton.clicked.connect(self.handle_md5sum)

    @Slot()
    def handle_md5sum(self):
        text = self.ui.textEdit.toPlainText()
        self.ui.textEdit.setText(text)
        digest = MD5.md5me(text)
        self.ui.lineEdit.setText(digest)
        self.ui.lineEdit.setVisible(True)


class MainWindow(QMainWindow):

    def __init__(self):
        super().__init__()
        # ??????ui???????????????????????????
        self.ui = Ui_MainWindow()

        cipher_names = ['????????????', '??????????????????', '???????????????', '??????????????????', '?????????????????????',
                        '?????????????????????????????????', '?????????????????????????????????', 'Playfair?????????', '?????????????????????', '??????????????????', '?????????????????????', 'RC4', 'DES', 'RSA', 'ECC', 'MD5']
        # ???????????????
        self.ui.setupUi(self)
        self.ui.cipherListWidget.addItems(cipher_names)

        self.setWindowTitle('Crypto Toolkit')

        self.ui.mainSplitter.setSizes([10000, 30000])
        self.ui.cipherListWidget.currentRowChanged.connect(self.select_cipher)
        self.cipher_groupboxes = [CaesarWidget(),
                                  KeywordWidget(),
                                  AffineWidget(),
                                  MultiliteralWidget(),
                                  VigenereWidget(),
                                  AutokeyCipherWidget(),
                                  AutokeyPlainWidget(),
                                  PlayfairWidget(),
                                  PermutationWidget(),
                                  ColumnPermutationWidget(),
                                  DoubleColumnPermutationWidget(),
                                  RC4Widget(),
                                  DESWidget(),
                                  RSAWidget(),
                                  ECCWidget(),
                                  MD5Widget()]

    @Slot(int)
    def select_cipher(self, cur):
        self.ui.mainSplitter.replaceWidget(1, self.cipher_groupboxes[cur])


if __name__ == '__main__':
    app = QApplication([])
    w = MainWindow()
    w.show()
    app.exec_()
