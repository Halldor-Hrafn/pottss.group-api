CREATE TABLE IF NOT EXISTS public.spells (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    rank SMALLINT NOT NULL,
    range TEXT NULL,
    area TEXT NULL,
    duration TEXT NULL,
    actions SMALLINT NOT NULL,
    components TEXT NULL,
    defense TEXT NULL,

    CONSTRAINT spells_pk PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.traits (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,

    CONSTRAINT traits_pk PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.spell_traits (
    spell_id UUID NOT NULL,
    trait_id UUID NOT NULL,

    CONSTRAINT spell_traits_pk PRIMARY KEY (spell_id, trait_id),
    CONSTRAINT spell_traits_spell_id_fk FOREIGN KEY (spell_id) REFERENCES public.spells (id),
    CONSTRAINT spell_traits_trait_id_fk FOREIGN KEY (trait_id) REFERENCES public.traits (id)
);

CREATE TABLE IF NOT EXISTS public.equipment (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    name TEXT NOT NULL,
    rarity TEXT NOT NULL,
    item_category TEXT NOT NULL,
    item_subcategory TEXT NULL,
    level SMALLINT NOT NULL,
    price TEXT NOT NULL,
    bulk TEXT NOT NULL,
    usage TEXT NOT NULL,
    json_data JSONB NOT NULL,

    CONSTRAINT equipment_pk PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.equipment_traits (
    equipment_id UUID NOT NULL,
    trait_id UUID NOT NULL,

    CONSTRAINT equipment_traits_pk PRIMARY KEY (equipment_id, trait_id),
    CONSTRAINT equipment_traits_equipment_id_fk FOREIGN KEY (equipment_id) REFERENCES public.equipment (id),
    CONSTRAINT equipment_traits_trait_id_fk FOREIGN KEY (trait_id) REFERENCES public.traits (id)
);

CREATE TABLE IF NOT EXISTS public.equipment_bonuses (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    equipment_id UUID NOT NULL,
    ability_skill TEXT NOT NULL,
    bonus TEXT NOT NULL,
    category TEXT NOT NULL,
    subcategory TEXT NULL,
    consumable BOOLEAN NOT NULL,
    note TEXT NULL

    CONSTRAINT equipment_bonuses_pk PRIMARY KEY (id),
    CONSTRAINT equipment_bonuses_equipment_id_fk FOREIGN KEY (equipment_id) REFERENCES public.equipment (id)
);

INSERT INTO public.traits (name, description) VALUES ('Cantrip', 'A spell you can cast at will that is automatically heightened to half your level rounded up.');

INSERT INTO public.spells (name, description, rank, range, area, duration, actions, components) VALUES ('Approximate',
 'Your magic quickly flows over an area to help you count and catalog. Name a particular type of object you are looking for within the area. You gain an instant estimate of the quantity of the chosen objects that are clearly visible within the target area. The number is rounded to the largest digit. For example, you could look at a pile of 180 copper coins, and you would learn that it held about 200 coins, but you couldn"t determine there were exactly 180 coins.\nThe type of object you name can be as specific or general as you like—“dented copper coins” is as viable as “coins”—but the distinguishing features must be obvious at a glance, and the spell is automatically fooled by objects disguised as other objects. For instance, the spell would register copper coins plated in gold as gold coins, not copper coins.',
  0, '10 feet', '1 cubic foot', 0, 2, 'somatic, verbal');

INSERT INTO public.spell_traits (spell_id, trait_id) VALUES ((SELECT id FROM public.spells WHERE name = 'Approximate'), (SELECT id FROM public.traits WHERE name = 'Cantrip'));

SELECT * FROM public.spells;
