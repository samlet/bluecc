from typing import Text

def resource_path(file_name) -> Text:
    """
    >>> from bluefin.conf import resource_path
    >>> resource_path('sagas_conf.json')

    :param file_name:
    :return:
    """
    import pkg_resources
    return pkg_resources.resource_filename(__name__, file_name)

def resource_json(file_name):
    import json_utils
    return json_utils.read_json_file(resource_path(file_name))
