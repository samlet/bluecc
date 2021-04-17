import streamlit as st
import json

# from interacts.common import display_lang_selector
from bluefin.interacts.sl_utils import write_styles
# from interacts.tracker_streamlit import enable_streamlit_tracker
from bluefin.procs.emitter import backend

# enable_streamlit_tracker()
write_styles()

def display_srv_selector(all_services):
    keys=list(all_services.keys())
    idx_en=keys.index("Perform find item")
    srv = st.sidebar.selectbox(
        'Which service do you choose?',
        list(keys),
        index=idx_en
    )

    cur_srv=all_services[srv]
    return cur_srv

def sidebar():
    srvs={"Perform find item":"performFindItem"}
    cur_srv=display_srv_selector(srvs)
    return cur_srv

default_request="""{
	"entityName":"OrderItem",	
	"inputFields":{
		"orderId":"WSCO10010"
	}
}"""

def main():
    st.title("Composer")
    st.text('Compose workflows')
    service=sidebar()
    # service = st.text_input('Service name', 'performFindItem')
    service = st.text_input('Service name', service)
    st.write('The current service is', service)
    json_str = st.text_area(label="Request parameters to execute",
                            value=default_request,
                            height=180,
                            )

    if len(service.strip())>0:
        resp = backend.invoke_srv(service.strip(), json.loads(json_str))
        st.markdown(f"service response status code: *{resp.status_code}*")
        try:
            json_obj = resp.json()
            st.json(json_obj)
        except ValueError as e:
            print("not json format")
            st.write(resp.text)


if __name__ == '__main__':
    main()

