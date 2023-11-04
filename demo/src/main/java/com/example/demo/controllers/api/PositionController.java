package com.example.demo.controllers.api;


import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;




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
}
