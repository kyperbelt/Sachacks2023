import csv
import json


base_url = "https://www.onetonline.org/link/table/details/.csv?fmt=csv"

categories = {
    "Tasks": "tk",
    "Technology Skills": "tc",
    "Work Activities": "wa",
    "Detailed Work Activities": "dw",
    "Work Context": "cx",
    "Job Zone": "jz"
}


def generate_link(category: str, job_id: str) -> str:
    if len(category.split(' ')) > 1:
        return f'{base_url[0:46]}{categories.get(category)}/{job_id}/{category.replace(" ", "_")}_{job_id}{base_url[46:len(base_url)]}'
    return f'{base_url[0:46]}{categories.get(category)}/{job_id}/{category}_{job_id}{base_url[46:len(base_url)]}'


def create_json_file():
    data_set = {}

    with open('All_Occupations.csv', newline='') as csv_file:
        reader = csv.reader(csv_file)
        next(reader)

        for data in reader:
            job_dict = {}
            job_title = data[2]
            job_id = data[1]
            job_dict['id'] = job_id
            url_list = []

            for category in categories:
                url_list.append(generate_link(category, job_id))

            job_dict['url'] = url_list
            data_set[job_title] = job_dict

    json_data_set = json.dumps(data_set, indent=4)

    with open("data_set.json", "w") as json_file:
        json_file.write(json_data_set)


if __name__ == '__main__':
    create_json_file()