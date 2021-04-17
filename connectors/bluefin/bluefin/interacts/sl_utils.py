import re
import streamlit as st

def write_styles():
    st.write("<style>red{color:red} orange{color:orange} "
             "yellow{color:yellow} green{color:green} "
             "blue{color:blue} purple{color:purple} "
             "cyan{color:blue} magenta{color:magenta} "
             "</style>", unsafe_allow_html=True)
