Feature: Project

    As a user when I want to create a Godot and I run the create command so I should get a project generated

    Scenario: Create Project
        Given that I wanted to create a project
        When I run the command to create a project
        Then I should have a project folder