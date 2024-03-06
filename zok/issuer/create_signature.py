# 조크라테스 pycrypto 사용 중이나, RustZoCrypto 마이그레이션 후 제거 예정

import hashlib
import json
from zokrates_pycrypto.eddsa import PrivateKey, PublicKey
from zokrates_pycrypto.field import FQ
from zokrates_pycrypto.utils import write_signature_for_zokrates_cli

def read_witness_values(file_path):
    # JSON 파일에서 값 읽기
    with open(file_path, 'r') as f:
        data = json.load(f)
    return [int(x) for x in data]

def write_signature_for_zokrates_cli(sig, msg, path):
    "Writes the input arguments for verifyEddsa in the ZoKrates stdlib to file."
    sig_R, sig_S = sig
    args = [sig_R.x, sig_R.y, sig_S]
    args = " ".join(map(str, args))

    with open(path, "w+") as file:
        for l in args:
            file.write(l)

if __name__ == "__main__":
    # 사용자 입력을 받음
    raw_msg = read_witness_values('witness_values.json')
    # 각 정수를 4바이트 바이트로 변환하고 하나의 바이트 배열로 합침
    msg_bytes = b''.join(int.to_bytes(x, length=4, byteorder='big', signed=False) for x in raw_msg)
    # print(msg_bytes)

    # Seeded for debug purpose
    key = FQ(1997011358982923168928344992199991480689546837621580239342656433234255379025)
    sk = PrivateKey(key)
    sig = sk.sign(msg_bytes)
    
    pk = PublicKey.from_private(sk)
    # is_verified = pk.verify(sig, msg_bytes)
    # print(is_verified)

    write_signature_for_zokrates_cli(sig, msg_bytes, 'signature')


    with open('pk', "w+") as file:
        for l in [str(pk.p.x.n)," ", str(pk.p.y.n)]:
            print(l)
            file.write(l)