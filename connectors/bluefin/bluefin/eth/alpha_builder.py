import os
from brownie import *

proj = project.load(os.path.expanduser('~/alpha'), name="alpha")
proj.load_config()

# from brownie.project.TokenProject import *
# from brownie.project.TokenProject import SimpleStore
from brownie.project import alpha

network.connect('development')

class AlphaBuilder(object):
    def objects(self):
        """
        $ python -m bluefin.eth.alpha_builder objects
        :return:
        """
        entries=dict(proj).keys()
        for i, entry in enumerate(entries):
            print(i, entry)

    def tests(self):
        """
        $ python -m bluefin.eth.alpha_builder tests
        :return:
        """
        f = alpha.MappingExample.deploy({'from': accounts[0]})
        print(f"contract address {f.address}, 1.amount -> {f.amounts(1)}")
        f.setAmount(1, 5)
        print(f"now 1.amount -> {f.amounts(1)}")

alpha_builder=AlphaBuilder()

if __name__ == '__main__':
    import fire
    fire.Fire(alpha_builder)




