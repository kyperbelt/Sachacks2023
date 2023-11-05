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

    @CrossOrigin
    @PostMapping("/getcareerskills")
    public PositionResponse test(PositionRequest request)
    {
        ArrayList<String> technologySkills = new ArrayList<>(Arrays.asList("1", "2"));

        return PositionResponse.builder()
                .technologySkills(technologySkills)
                .build();
    }
}
