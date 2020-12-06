Feature: User Registration

    Scenario: Visitor can register as a Truist user.
        Given I want to register for Truist
        When I click the "Sign Up" button
        Then I am taken to the Registration Page
