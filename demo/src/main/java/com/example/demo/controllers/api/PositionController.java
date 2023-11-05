package com.example.demo.controllers.api;

import com.example.demo.PositionRequest;
import com.example.demo.PositionResponse;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;

import java.util.ArrayList;
import java.util.Arrays;

@RestController
public class PositionController {
    
    @GetMapping
    @ResponseStatus(value= HttpStatus.OK)
    public void GetSomething(){

    }

    @PostMapping
    @ResponseStatus(value= HttpStatus.OK)
    public void PostSomething(String something){

    }

    @PutMapping
    @ResponseStatus(value= HttpStatus.OK)
    public void PutSomething(String something){

    }

    @DeleteMapping
    @ResponseStatus(value= HttpStatus.OK)
    public void DeleteSomething(){

    }

    @PostMapping("/test")
    public PositionResponse test(PositionRequest request)
    {
        ArrayList<String> tasks = new ArrayList<>(Arrays.asList("1", "2"));
        ArrayList<String> technologySkills = new ArrayList<>(Arrays.asList("3", "4"));
        ArrayList<String> workActivities = new ArrayList<>(Arrays.asList("5", "6"));
        ArrayList<String> detailedWorkActivities = new ArrayList<>(Arrays.asList("7", "8"));
        ArrayList<String> workContext = new ArrayList<>(Arrays.asList("9", "0"));
        ArrayList<String> jobZone = new ArrayList<>(Arrays.asList("-", "+"));

        return PositionResponse.builder()
                .tasks(tasks)
                .technologySkills(technologySkills)
                .workActivities(workActivities)
                .detailedWorkActivities(detailedWorkActivities)
                .workContext(workContext)
                .jobZone(jobZone)
                .build();
    }

}
