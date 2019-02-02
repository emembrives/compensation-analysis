from bs4 import BeautifulSoup
import requests

LEETCHI_MEDICAL_HOMEPAGE = "https://www.leetchi.com/fr/cagnottes/medical"

def download_index() -> None:
    r = requests.get(LEETCHI_MEDICAL_HOMEPAGE)
    html = BeautifulSoup(r.text)
    for card in html.find_all('.fundraising-card'):
        
    