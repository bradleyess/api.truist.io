Feature: Authentication with Stripe

    Scenario: Truist API can authenticate with Stripe API
        Given I am connecting to the Stripe API
        When I create a Stripe client
        Then I can connect to Stripe
