import logging

logger = logging.getLogger(__name__)

class Emitter(object):
    def info(self):
        """
        $ python -m bluefin.procs.emitter info
        :return:
        """
        logger.info(".. emitter")

if __name__ == '__main__':
    import fire
    fire.Fire(Emitter)

