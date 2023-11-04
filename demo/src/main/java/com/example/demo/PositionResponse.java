package com.example.demo;

import java.util.ArrayList;

import lombok.Getter;
import lombok.Setter;

public class PositionResponse {
    @Getter
    @Setter
    private ArrayList <String> tasks;
    @Getter
    @Setter
    private ArrayList <String> technologySkills;
    @Getter
    @Setter
    private ArrayList <String> workActivities;
    @Getter
    @Setter
    private ArrayList <String> detailedWorkActivities;
    @Getter
    @Setter
    private ArrayList <String> workContext;
    @Getter
    @Setter
    private ArrayList <String> jobZone;

    
}
