package com.example.demo;

import lombok.Getter;
import lombok.Setter;


public class PositionRequest {
    @Getter
    @Setter
    private String title;

    
    public PositionRequest(String title){
        this.title=title;
    }

}
