package com.example.demo;

import lombok.Getter;
import lombok.Setter;
import java.util.ArrayList;


public class PositionRequest {
    @Getter
    @Setter
    private ArrayList<String> titles;
    
    public PositionRequest(ArrayList<String> titles){
        this.titles = titles;
    }

}
