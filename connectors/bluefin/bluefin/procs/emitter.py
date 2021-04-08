from typing import Text, Any, Dict, List, Union, Optional, Tuple
import pandas as pd
import json
import logging
import requests
import urllib3

urllib3.disable_warnings()
logger = logging.getLogger(__name__)

class Backend(object):
    def __init__(self):
        self.ofbiz_srvs="https://localhost:8443/rest/services"

    def invoke_srv(self, srv, paras):
        url = f"{self.ofbiz_srvs}/{srv}"
        r = requests.post(url, json=paras, verify=False)
        return r

backend=Backend()

def to_df(list_of_tuples:List[Tuple], columns:List[Text]) -> pd.DataFrame:
    return pd.DataFrame(list_of_tuples, columns=columns)

def print_df(df):
    from tabulate import tabulate
    print(tabulate(df, headers='keys', tablefmt='psql'))

def print_rs(rs, cols):
    print_df(to_df(rs, cols))

class Emitter(object):
    def info(self):
        """
        $ python -m bluefin.procs.emitter info
        :return:
        """
        logger.info(".. emitter")

    def find_list(self, ent, fld, fld_val, rows=10):
        """
        $ python -m bluefin.procs.emitter find_list Uom uomId USD
        :param ent:
        :param fld:
        :param fld_val:
        :return:
        """
        resp = backend.invoke_srv("performFindList", {
            "entityName": ent,
            "viewIndex": 0,
            "viewSize": rows,
            "inputFields": {
                fld: fld_val
            }
        })

        print(resp.status_code)
        print(json.dumps(resp.json(), indent=2))

    def list_uom(self, max_rows):
        """
        $ python -m bluefin.procs.emitter list_uom 10
        :param ent:
        :return:
        """
        resp = backend.invoke_srv("findCc", {
            "entityName": "Uom",
            "maxRows": max_rows
        })

        if resp.status_code == 200:
            data = resp.json()['data']
            print_rs([(r['uomId'], r['description']) for r in data['result']],
                     ['id', 'desc'])
        else:
            print(resp.status_code)
            print(json.dumps(resp.json(), indent=2))


if __name__ == '__main__':
    import fire
    fire.Fire(Emitter)

