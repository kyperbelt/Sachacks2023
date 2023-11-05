import tensorflow as tf
import pandas as pd
import numpy as np
import json
from flask import Flask, request

app = Flask(__name__)

@app.route("/", methods=['GET'])
def hello_world():
    args = request.args
    skills = args.getlist('skill')
    prod = Prod()
    return prod.get_job_list(skills)
    

class Prod:
    
    # load best model
    def __init__(self, model_path : str = './best_weights'):
        with open('jlist.json', 'r') as f:
            blob = json.loads(f.read())
            self.list_of_all_skills = blob['list_of_all_skills']
            self.list_of_all_jobs = blob['list_of_all_jobs']

        self.model = self.create_model()
        self.model.load_weights(model_path)


    
    def create_model(self) -> tf.keras.models.Sequential:
        skill_count = 4245
        job_count = 918
        model = tf.keras.models.Sequential()
        model.add(tf.keras.layers.Dense(skill_count, input_dim=skill_count))
        model.add(tf.keras.layers.Dense(int(skill_count/4), activation='relu') )
        model.add(tf.keras.layers.Dense(int(skill_count/8), activation='relu') )

        # Output
        model.add(tf.keras.layers.Dense(job_count, activation='softmax'))
        model.compile(loss='categorical_crossentropy', optimizer=tf.keras.optimizers.legacy.Adam())
        return model

        
    def get_job_list(self, skills: list[str], job_count: int = 10):
        # One hot encode skills
        arr = np.array([[1 if skill in skills else 0 for skill in self.list_of_all_skills]])
        
        # Send it to the model
        pred = self.model.predict(arr)
        
        # Create a dataframe and series
        df = pd.DataFrame(pred, columns=self.list_of_all_jobs)
        series = pd.Series(df.loc[0])

        # Take the largest job count
        rv = pd.Series.nlargest(series, n = job_count).to_dict()
        
        # Return the list of carrers
        return list(rv)

    
