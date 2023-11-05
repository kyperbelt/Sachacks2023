package com.example.demo;

import java.util.ArrayList;

import lombok.Getter;
import lombok.Setter;
import lombok.Builder;

@Builder
public class PositionResponse {
    @Getter
    @Setter
    private ArrayList <String> technologySkills;
}
