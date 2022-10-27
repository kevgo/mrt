Feature: list subfolders matching a condition

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: it displays the folders where the condition returns 0

    Scenario: happy path
      When running "m list ls go.mod"
      Then it prints:
        """
        ...

        2/3 folders match:
        1. go
        2. go_node
        """
      And it returns "success"
      And I am now in the "simple" example folder
      And there is no saved state