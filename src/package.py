class Package:
    def __init__(self, name):
        self.name = name
        self.installed = False
        self.req_deps = []
        self.rec_deps = []
        self.opt_deps = []
        self.req_run_deps = []
        self.rec_run_deps = []
        self.opt_run_deps = []
        self.has_req_deps = 'None'
        self.has_rec_deps = 'None'
        self.has_opt_deps = 'None'
        self.has_req_run_deps = 'None'
        self.has_rec_run_deps = 'None'
        self.has_opt_run_deps = 'None'

    def __str__(self):
        return self.name

    @property
    def entry(self):
        return self.__str__()