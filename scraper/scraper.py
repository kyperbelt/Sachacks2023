import csv
import json
import requests

career_category_base_url = "https://www.onetonline.org/find/career/.csv?c=&fmt=csv"
career_datum_base_url = "https://www.onetonline.org/link/table/details/.csv?fmt=csv&s=s"

career_data = {
    "Tasks": "tk",
    "Technology Skills": "tc",
    "Work Activities": "wa",
    "Detailed Work Activities": "dw",
    "Work Context": "cx",
    "Job Zone": "jz"
}

career_categories = ["Agriculture Food Natural Resources", "Architecture Construction",
                     "Arts Audio Video Technology Communications", "Business Management Administration",
                     "Education Training", "Finance", "Government Public Administration", "Health Science",
                     "Hospitality Tourism", "Human Services", "Information Technology",
                     "Law Public Safety Corrections Security", "Manufacturing", "Marketing",
                     "Science Technology Engineering Mathematics", "Transportation Distribution Logistics"]


def generate_one_career_category_link(category: str, id: int) -> str:
    return f'{career_category_base_url[0:39]}{category.replace(" ", "_")}{career_category_base_url[39:46]}{id}{career_category_base_url[46:len(career_category_base_url)]}'


def generate_all_career_category_links() -> list:
    career_category_list = []

    for i, career in enumerate(career_categories):
        career_category_list.append(generate_one_career_category_link(career, i + 1))
    return career_category_list


def generate_one_career_datum_link(category: str, job_id: str) -> str:
    return f'{career_datum_base_url[0:46]}{career_data.get(category)}/{job_id}/{category.replace(" ", "_")}_{job_id}{career_datum_base_url[46:len(career_datum_base_url)]}'


if __name__ == '__main__':
    career_links = generate_all_career_category_links()
    data_set = {}

    for i, career in enumerate(career_categories):
        with requests.Session() as outer_session:
            job_dict = {}
            csv_file = outer_session.get(career_links[i])
            decoded_csv_file = csv_file.content.decode('utf-8')
            csv_lines = csv.reader(decoded_csv_file.splitlines(), delimiter=',')
            next(csv_lines)

            for outer_row in csv_lines:
                technology_list = []
                empty_career = None
                job_id = outer_row[1]
                job_title = outer_row[2]
                if "All Other" not in job_title:
                    with requests.Session() as inner_session:
                        temp_link = generate_one_career_datum_link("Technology Skills", job_id)
                        inner_csv_file = inner_session.get(temp_link)
                        inner_decoded_csv_file = inner_csv_file.content.decode('utf-8')
                        inner_csv_lines = csv.reader(inner_decoded_csv_file.splitlines(), delimiter=',')
                        next(inner_csv_lines)

                        for inner_row in inner_csv_lines:
                            if len(inner_row) == 3 and inner_row[2]:
                                technology_list.append(inner_row[2])
                                empty_career = inner_row[2]

                    if empty_career not in " shrink-to-fit=no\">":
                        job_dict[job_title] = technology_list
            data_set[career] = job_dict

    json_data_set = json.dumps(data_set, indent=4)
    with open("new_data_set.json", "w") as json_file:
        json_file.write(json_data_set)

