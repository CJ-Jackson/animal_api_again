UPDATE animal
SET species=:species,
    description=:description
WHERE id = :id;