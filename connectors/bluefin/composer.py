from typing import List

import streamlit as st
import json

# from interacts.common import display_lang_selector
from bluefin.interacts.sl_utils import write_styles
# from interacts.tracker_streamlit import enable_streamlit_tracker
from bluefin.procs.emitter import backend
from streamlit_ace import st_ace

# enable_streamlit_tracker()
write_styles()


def app_panels():
    option = st.sidebar.selectbox(
        'Which Application Panel?',
        ('Service Meta', 'Main Entity', 'Service Requests', 'Entites'))
    st.sidebar.markdown(f'Current: *{option}*')
    return option


def display_srv_selector(all_services):
    keys = list(all_services.keys())
    idx_en = keys.index("Perform find item")
    srv = st.sidebar.selectbox(
        'Which service do you choose?',
        list(keys),
        index=idx_en
    )

    cur_srv = all_services[srv]
    return cur_srv


def sidebar():
    filter = "find"
    filter = st.sidebar.text_input('Filter', filter)
    srvs = {"Perform find item": "performFindItem"}
    cur_srv = display_srv_selector(srvs)
    return cur_srv


default_request = """{
	"entityName":"OrderItem",	
	"inputFields":{
		"orderId":"WSCO10010"
	}
}"""


def service_requests():
    service = sidebar()
    # service = st.text_input('Service name', 'performFindItem')
    service = st.text_input('Service name', service)
    st.write('The current service is', service)
    # json_str = st.text_area(label="Request parameters to execute",
    #                         value=default_request,
    #                         height=180,
    #                         )
    json_str = st_ace(language="json",
                      font_size=11,
                      # theme="chrome",
                      theme="chaos",
                      keybinding="sublime",
                      value=default_request,
                      height=180,
                      )
    if st.button("Execute Request"):
        if len(service.strip()) > 0:
            resp = backend.invoke_srv(service.strip(), json.loads(json_str))
            st.markdown(f"service response status code: *{resp.status_code}*")
            try:
                json_obj = resp.json()
                st.json(json_obj)
            except ValueError as e:
                print("not json format")
                st.write(resp.text)


THEMES = [
    "ambiance", "chaos", "chrome", "clouds", "clouds_midnight", "cobalt", "crimson_editor", "dawn",
    "dracula", "dreamweaver", "eclipse", "github", "gob", "gruvbox", "idle_fingers", "iplastic",
    "katzenmilch", "kr_theme", "kuroir", "merbivore", "merbivore_soft", "mono_industrial", "monokai",
    "nord_dark", "pastel_on_dark", "solarized_dark", "solarized_light", "sqlserver", "terminal",
    "textmate", "tomorrow", "tomorrow_night", "tomorrow_night_blue", "tomorrow_night_bright",
    "tomorrow_night_eighties", "twilight", "vibrant_ink", "xcode"
]

@st.cache(allow_output_mutation=True, suppress_st_warning=True)
def get_model_services(main_ent: str) -> List[str]:
    from sagas.ofbiz.services import oc
    services = oc.all_service_names()
    result=[]
    for srv in services:
        model = oc.service_model(srv)
        ent=model.getDefaultEntityName()
        if ent==main_ent:
            result.append(srv)
    return result

def main_entity(main_ent):
    from sagas.ofbiz.entities import OfEntity as e
    from sagas.ofbiz.services import OfService as s, create_service_data_frame, MetaService

    show_entity_flds=st.sidebar.checkbox(label="Show entity fields", value=False)

    # srv_name='createPerson'
    srvs=get_model_services(main_ent)
    for srv_name in srvs:
        model=model=MetaService(srv_name).model
        default_ent=model.getDefaultEntityName()
        if default_ent!='':
            st.markdown(f"*{srv_name}*, has default entity **{default_ent}**")
        else:
            st.markdown(f"*Service Meta*: {srv_name}")
        st.text(f"{model.getDescription()}")
        df_srv=create_service_data_frame(srv_name, show_internal=False,
                                         show_entity_flds=show_entity_flds)
        st.table(df_srv)

    st.markdown("*Entity Meta*")
    df_ent=e('meta').Person
    st.dataframe(df_ent)

    st.markdown("*Entity Relations*")
    df_rels=e('relations').Person
    st.dataframe(df_rels)

def service_meta(srv_name):
    from sagas.ofbiz.services import OfService as s, create_service_data_frame, MetaService
    model = model = MetaService(srv_name).model
    st.markdown(f"*Service Meta*: {srv_name}")
    st.text(f"{model.getDescription()}")
    df_srv = create_service_data_frame(srv_name)
    st.table(df_srv)

def main():
    st.title("Composer")
    st.text('Compose workflows')
    panel = app_panels()

    if panel == 'Service Requests':
        service_requests()
    elif panel=='Main Entity':
        main_entity(main_ent=st.sidebar.text_input(label='Main entity name', value='Person'))
    elif panel=='Service Meta':
        service_meta(srv_name=st.sidebar.text_input(label='Service Name',
                                                    value='createPerson'))
    else:
        content = st_ace(language="json",
                         font_size=11,
                         theme=st.sidebar.selectbox("Theme", options=THEMES, index=31),
                         value=default_request,
                         )


if __name__ == '__main__':
    main()
